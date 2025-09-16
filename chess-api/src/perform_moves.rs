
use crate::legal_moves::bishop_moves;
use crate::legal_moves::king_moves;
use crate::legal_moves::knight_moves;
use crate::legal_moves::pawn_moves;
use crate::legal_moves::queen_moves;
use crate::legal_moves::rook_moves;
use crate::state::GameState;
use crate::bitboards::Board;

const SECOND_RANK: u64 = 0b1111111100000000; 
const SEVENTH_RANK: u64 = 0xFF000000000000;
const FIRST_RANK: u64 = 0x00000000000000FF;
const EIGHT_RANK: u64 = 0xFF00000000000000;

use std::io::{self, Write}; // take input, got some help from Arvid Kristofferson on how to take input in rust

pub fn is_legal(cur_square: i8, target_square: i8, state: &GameState) -> bool {

    if cur_square < 0 || cur_square > 63 || target_square < 0 || target_square > 63 {
        return false;
    }

    let cur_mask: u64 = 1<<cur_square;
    let target_mask: u64 = 1<<target_square;

    // https://users.rust-lang.org/t/mutex-lock-twice/88414
    // can't lock Mutex BOARD twice, but since it's also locked in pawn_moves(), bishop_moves() etc.
    // the lock needs to be dropped before calling these functions
    // https://users.rust-lang.org/t/when-is-drop-called-on-mutex/6571/2


    let board = &state.board;
    let mut legal_piece_movement: bool = false;


    if ((cur_mask & board.white_pawns) != 0) && state.white_to_move {
        legal_piece_movement = (pawn_moves(cur_mask, &state) & target_mask) != 0;
    }

    if ((cur_mask & board.white_bishops) != 0) && state.white_to_move {
        legal_piece_movement = (bishop_moves(cur_mask, &state) & target_mask) != 0;
    }

    if ((cur_mask & board.white_knights) != 0) && state.white_to_move {
        legal_piece_movement = (knight_moves(cur_mask, &state) & target_mask) != 0;
    }

    if ((cur_mask & board.white_rooks) != 0) && state.white_to_move {
        legal_piece_movement = (rook_moves(cur_mask, &state) & target_mask) != 0;
    }

    if ((cur_mask & board.white_queens) != 0) && state.white_to_move {
        legal_piece_movement = (queen_moves(cur_mask, &state) & target_mask) != 0;
    }

    if ((cur_mask & board.white_king) != 0) && state.white_to_move {
        legal_piece_movement = (king_moves(cur_mask, &state, true) & target_mask) != 0;
    }
 
    if ((cur_mask & board.black_pawns) != 0) && !state.white_to_move {
        legal_piece_movement = (pawn_moves(cur_mask, &state) & target_mask) != 0;
    }

    if ((cur_mask & board.black_bishops) != 0) && !state.white_to_move {
        legal_piece_movement = (bishop_moves(cur_mask, &state) & target_mask) != 0;
    }

    if ((cur_mask & board.black_knights) != 0) && !state.white_to_move {
        legal_piece_movement = (knight_moves(cur_mask, &state) & target_mask) != 0;
    }

    if ((cur_mask & board.black_rooks) != 0) && !state.white_to_move {
        legal_piece_movement = (rook_moves(cur_mask, &state) & target_mask) != 0;
    }

    if ((cur_mask & board.black_queens) != 0) && !state.white_to_move {
        legal_piece_movement = (queen_moves(cur_mask, &state) & target_mask) != 0;
    }

    if ((cur_mask & board.black_king) != 0) && !state.white_to_move {
        legal_piece_movement = (king_moves(cur_mask, &state, true) & target_mask) != 0;
    }

    if !legal_piece_movement {
        return false;
    }

    let simulated_state = &mut simulate_make_move(cur_square, target_square, state);

    simulated_state.white_to_move = !simulated_state.white_to_move;

    if is_check(&simulated_state) {
        return false; // white king would be in check
    }

    return true;
}

pub fn simulate_make_move(cur_square: i8, target_square: i8, state: &GameState) -> GameState{ 
    // doesn't actually perform the move, just "pretends" to make the move -> for checks of check, checkmate etc.

    // this is slow and requires to make tons of clones, BUT easier to implement for now.
    // Potential improvement: Create an undo_move() that allows you to keep changing the global board, and then reverse the changes if needed.

    let mut temp_state = state.clone(); // independent copy
    let cur_mask = 1 << cur_square;
    let target_mask = 1 << target_square;

    let temp_board = &mut temp_state.board;

    // if capture
    capture(target_mask, temp_board);

    // add piece to target square
    fill_target_square(cur_mask, target_mask, temp_board);

     // remove piece from current square
    empty_current_square(cur_mask, temp_board);


    return temp_state;
}

pub fn make_move(cur_square: i8, target_square: i8, state: &mut GameState, stop_reset: bool) -> bool{

    // stop_reset is set to true during testing

    let cur_mask: u64 = 1<<cur_square;
    let target_mask: u64 = 1<<target_square;

    if !is_legal(cur_square, target_square, &state) {
        print!("{}", "Invalid move.\n");
        return false;
    }

    let board = &mut state.board;


    // if capture
    capture(target_mask, board);

    // add piece to target square
    fill_target_square(cur_mask, target_mask, board);

     // remove piece from current square
    empty_current_square(cur_mask, board);


    en_passant(target_mask, state);
    update_en_passant_square(cur_square, target_square, state);

    castle(cur_square, target_square, state);
    update_castling_rights(cur_square, state);

    promotion(target_square, state);


    // CHECK FOR CHECKMATE LAST

    if is_checkmate_stalemate(state) {
        if is_check(state){
            print!("CHECKMATE\n");
        } else {
            print!("STALEMATE\n");
        }

        if !stop_reset {
            state.reset(); // reset the game
        }
        return true;
    }


    // toggle turns
    state.white_to_move = !state.white_to_move;

    return true;

}

pub fn capture(target_mask: u64, board: &mut Board) {

    if (target_mask & board.black_occupied) != 0 {
        board.black_occupied &= !target_mask;
        board.black_pawns &= !target_mask;
        board.black_king &= !target_mask;
        board.black_queens &= !target_mask;
        board.black_knights &= !target_mask;
        board.black_rooks &= !target_mask;
        board.black_bishops &= !target_mask;
    }
    if (target_mask & board.white_occupied) != 0 {
        board.white_occupied &= !target_mask;
        board.white_pawns &= !target_mask;
        board.white_king &= !target_mask;
        board.white_queens &= !target_mask;
        board.white_knights &= !target_mask;
        board.white_rooks &= !target_mask;
        board.white_bishops &= !target_mask;
    }
}

pub fn fill_target_square(cur_mask: u64, target_mask: u64, board: &mut Board) {

    if (cur_mask & board.white_pawns) != 0 {
        board.white_occupied |= target_mask;
        board.white_pawns |= target_mask;
    }

    if (cur_mask & board.white_king) != 0 {
        board.white_occupied |= target_mask;
        board.white_king |= target_mask;
    }

    if (cur_mask & board.white_queens) != 0 {
        board.white_occupied |= target_mask;
        board.white_queens |= target_mask;
    }

    if (cur_mask & board.white_knights) != 0 {
        board.white_occupied |= target_mask;
        board.white_knights |= target_mask;
    }

    if (cur_mask & board.white_rooks) != 0 {
        board.white_occupied |= target_mask;
        board.white_rooks |= target_mask;
    }

    if (cur_mask & board.white_bishops) != 0 {
        board.white_occupied |= target_mask;
        board.white_bishops |= target_mask;
    }

    if (cur_mask & board.black_pawns) != 0 {
        board.black_occupied |= target_mask;
        board.black_pawns |= target_mask;
    }

    if (cur_mask & board.black_king) != 0 {
        board.black_occupied |= target_mask;
        board.black_king |= target_mask;
    }

    if (cur_mask & board.black_queens) != 0 {
        board.black_occupied |= target_mask;
        board.black_queens |= target_mask;
    }

    if (cur_mask & board.black_knights) != 0 {
        board.black_occupied |= target_mask;
        board.black_knights |= target_mask;
    }

    if (cur_mask & board.black_rooks) != 0 {
        board.black_occupied |= target_mask;
        board.black_rooks |= target_mask;
    }

    if (cur_mask & board.black_bishops) != 0 {
        board.black_occupied |= target_mask;
        board.black_bishops |= target_mask;
    }

}

pub fn empty_current_square(cur_mask: u64, board: &mut Board) {
    board.white_occupied &= !cur_mask;
    board.white_pawns &= !cur_mask;
    board.white_king &= !cur_mask;
    board.white_queens &= !cur_mask;
    board.white_knights &= !cur_mask;
    board.white_rooks &= !cur_mask;
    board.white_bishops &= !cur_mask;

    board.black_occupied &= !cur_mask;
    board.black_pawns &= !cur_mask;
    board.black_king &= !cur_mask;
    board.black_queens &= !cur_mask;
    board.black_knights &= !cur_mask;
    board.black_rooks &= !cur_mask;
    board.black_bishops &= !cur_mask;
}

pub fn promotion(target_square: i8, state: &mut GameState){

    let target_mask = 1<<target_square;

    let promoted_pawns = if state.white_to_move {
        target_mask & state.board.white_pawns & EIGHT_RANK
    } else {
        target_mask & state.board.black_pawns & FIRST_RANK
    };

    if promoted_pawns == 0 {
        return; // no promoted pawn
    }

    loop {

        print!("PROMOTE pawn on square {} to (Q, R, B, N)\n", target_square);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let promotion_piece = input.trim().to_uppercase();

         // Remove the pawn from its bitboard
        if state.white_to_move {
            state.board.white_pawns &= !target_mask;
        } else {
            state.board.black_pawns &= !target_mask;
        }

        // Add the promoted piece
        let valid = match promotion_piece.as_str() {
            "Q" => {
                if state.white_to_move {
                    state.board.white_queens |= target_mask;
                } else {
                    state.board.black_queens |= target_mask;
                }
                true
            }
            "R" => {
                if state.white_to_move {
                    state.board.white_rooks |= target_mask;
                } else {
                    state.board.black_rooks |= target_mask;
                }
                true
            }
            "B" => {
                if state.white_to_move {
                    state.board.white_bishops |= target_mask;
                } else {
                    state.board.black_bishops |= target_mask;
                }
                true
            }
            "N" => {
                if state.white_to_move {
                    state.board.white_knights |= target_mask;
                } else {
                    state.board.black_knights |= target_mask;
                }
                true
            }
            _ => {
                println!("Invalid choice, please enter Q, R, B, or N.");
                false
            }
        };

        if valid {
                    break;
                }
    }

}

pub fn en_passant(target_mask: u64, state: &mut GameState) { // checks if the move made was en passant -> update opponent occupied squares

    if target_mask == state.en_passant_mask {

        if state.white_to_move {

            state.board.black_pawns &= !(target_mask >> 8);
            state.board.black_occupied &= !(target_mask >> 8);

        } else {

            state.board.white_pawns &= !(target_mask << 8);
            state.board.white_occupied &= !(target_mask << 8);
        }
    }


}

pub fn update_en_passant_square(cur_square: i8, target_square: i8, state: &mut GameState) {

    let cur_mask = 1<<cur_square;
    let target_mask: u64 = 1<<target_square;


    if (target_mask & state.board.white_pawns) != 0 {

        if (cur_mask & SECOND_RANK) != 0 && target_square-cur_square==16 {

            state.en_passant_mask = 1<< cur_square+8;
        }


    } else if (target_mask & state.board.black_pawns)!= 0 {
        
        if (cur_mask & SEVENTH_RANK) != 0 && cur_square-target_square==16 {
            
            state.en_passant_mask = 1<<(cur_square-8);
        }

    } else {
        state.en_passant_mask = 0;
    }

}

pub fn castle(cur_square: i8, target_square: i8, state: &mut GameState) {
    
    //let cur_mask = 1<<cur_square;
    let target_mask: u64 = 1<<target_square;

    if (target_mask & state.board.white_king) != 0 {

        if cur_square == 4 && target_square == 6 { // king-side castling

            // rook on h1 to f1
            state.board.white_rooks |= 1<<5;
            state.board.white_rooks &= !(1<<7);
        }

        else if cur_square == 4 && target_square == 2 { // queen-side castling

            // rook on a1 to d1
            state.board.white_rooks |= 1<<3;
            state.board.white_rooks &= !(1<<0);
        }


    } else if (target_mask & state.board.black_king) != 0 {

        if cur_square == 60 && target_square == 62  {// king-side

            // rook on h8 to f8
            state.board.black_rooks |= 1<<61;
            state.board.black_rooks &= !(1<<63);
        }

        else if cur_square == 60 && target_square == 58 { // queen-side

            // rook on a8 to c8
            state.board.black_rooks |= 1<<59;
            state.board.black_rooks &= !(1<<56);

        }

    }

}

pub fn update_castling_rights(cur_square: i8, state: &mut GameState) {

    if cur_square == 0 { // rook on a1
        state.white_can_castle_queenside = false;
    }
    else if cur_square == 4 { // king on e1
        state.white_can_castle_kingside = false;
        state.white_can_castle_queenside = false;
    }
    else if cur_square == 7 { // rook on h1
        state.white_can_castle_kingside = false;
    }
    else if cur_square == 56 { // rook on a8
        state.black_can_castle_queenside = false;
    }
    else if cur_square == 60 { // king on e8
        state.black_can_castle_kingside = false;
        state.black_can_castle_queenside = false;
    }
    else if cur_square == 63 { // rook on h8
        state.black_can_castle_kingside = false;
    }


} 



pub fn checked_squares(state: &GameState) -> u64 {


    let board = &state.board;

    if state.white_to_move {

        let mut all_targeted_squares: u64 = 0;

        all_targeted_squares |= pawn_moves(board.white_pawns, state);
        all_targeted_squares |= knight_moves(board.white_knights, state);
        all_targeted_squares |= bishop_moves(board.white_bishops, state);
        all_targeted_squares |= rook_moves(board.white_rooks, state);
        all_targeted_squares |= queen_moves(board.white_queens, state);
        all_targeted_squares |= king_moves(board.white_king, state, false);

        return all_targeted_squares;
    
    } else {

        let mut all_targeted_squares: u64 = 0;

        all_targeted_squares |= pawn_moves(board.black_pawns, state);
        all_targeted_squares |= knight_moves(board.black_knights, state);
        all_targeted_squares |= bishop_moves(board.black_bishops, state);
        all_targeted_squares |= rook_moves(board.black_rooks, state);
        all_targeted_squares |= queen_moves(board.black_queens, state);
        all_targeted_squares |= king_moves(board.black_king, state, false);

        return all_targeted_squares;

    }


}

pub fn is_check(state: &GameState) -> bool { // call before changing moves

    /* compute ALL potential moves for the color that just moved.
    Otherwise will not account for discovered checks!*/ 

    let board = &state.board;

    if state.white_to_move {
        return checked_squares(state) & board.black_king != 0;

    } else {
        return checked_squares(state) & board.white_king != 0;
    }

}


pub fn is_checkmate_stalemate(state: &mut GameState) -> bool {
    
    // try all possible moves
    // to_move is the player who just made a move. (to_move+1)%2 gives the other player's move

    let mut state_clone = state.clone();
    state_clone.white_to_move  = !state_clone.white_to_move;

    for cur_square in 0..64 {
        
        for target_square in 0..64 {

        
            if is_legal(cur_square, target_square, &state_clone) {
                let new_state = &mut simulate_make_move(cur_square, target_square, &state_clone);
                new_state.white_to_move = !new_state.white_to_move;
                if !is_check(&new_state) {
                    return false;
                }

            }
        }
    }
    return true;
}



