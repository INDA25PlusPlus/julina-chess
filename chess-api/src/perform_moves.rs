
use crate::legal_moves::bishop_moves;
use crate::legal_moves::king_moves;
use crate::legal_moves::knight_moves;
use crate::legal_moves::pawn_moves;
use crate::legal_moves::queen_moves;
use crate::legal_moves::rook_moves;
use crate::state::GameState;
use crate::bitboards::Board;
use crate::state::Color;
use crate::state::History;

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

    let side = state.side_to_move;

    let white_to_move = match side {
        Color::White => true,
        Color::Black => false,
    };


    // https://users.rust-lang.org/t/mutex-lock-twice/88414
    // can't lock Mutex BOARD twice, but since it's also locked in pawn_moves(), bishop_moves() etc.
    // the lock needs to be dropped before calling these functions
    // https://users.rust-lang.org/t/when-is-drop-called-on-mutex/6571/2


    let board = &state.board;
    let mut legal_piece_movement: bool = false;


    if ((cur_mask & board.white_pawns) != 0) && white_to_move {
        legal_piece_movement = (pawn_moves(cur_mask, &state, side) & target_mask) != 0;
    }

    if ((cur_mask & board.white_bishops) != 0) && white_to_move {
        legal_piece_movement = (bishop_moves(cur_mask, &state, side) & target_mask) != 0;
    }

    if ((cur_mask & board.white_knights) != 0) && white_to_move {
        legal_piece_movement = (knight_moves(cur_mask, &state, side) & target_mask) != 0;
    }

    if ((cur_mask & board.white_rooks) != 0) && white_to_move {
        legal_piece_movement = (rook_moves(cur_mask, &state, side) & target_mask) != 0;
    }

    if ((cur_mask & board.white_queens) != 0) && white_to_move {
        legal_piece_movement = (queen_moves(cur_mask, &state, side) & target_mask) != 0;
    }

    if ((cur_mask & board.white_king) != 0) && white_to_move {
        legal_piece_movement = (king_moves(cur_mask, &state, side,true) & target_mask) != 0;
    }
 
    if ((cur_mask & board.black_pawns) != 0) && !white_to_move {
        legal_piece_movement = (pawn_moves(cur_mask, &state, side) & target_mask) != 0;
    }

    if ((cur_mask & board.black_bishops) != 0) && !white_to_move {
        legal_piece_movement = (bishop_moves(cur_mask, &state, side) & target_mask) != 0;
    }

    if ((cur_mask & board.black_knights) != 0) && !white_to_move {
        legal_piece_movement = (knight_moves(cur_mask, &state, side) & target_mask) != 0;
    }

    if ((cur_mask & board.black_rooks) != 0) && !white_to_move {
        legal_piece_movement = (rook_moves(cur_mask, &state, side) & target_mask) != 0;
    }

    if ((cur_mask & board.black_queens) != 0) && !white_to_move {
        legal_piece_movement = (queen_moves(cur_mask, &state, side) & target_mask) != 0;
    }

    if ((cur_mask & board.black_king) != 0) && !white_to_move {
        legal_piece_movement = (king_moves(cur_mask, &state, side, true) & target_mask) != 0;
    }

    if !legal_piece_movement {
        return false;
    }

    // let simulated_state = &mut simulate_make_move(cur_square, target_square, state);

    // if is_check(&simulated_state, side.opposite()) {
    //     return false; // white king would be in check
    // }

    return true;
}

// pub fn simulate_make_move(cur_square: i8, target_square: i8, state: &GameState) -> GameState{ 
//     // doesn't actually perform the move, just "pretends" to make the move -> for checks of check, checkmate etc.

//     // this is slow and requires making lots of clones, BUT easier to implement for now.
//     // Potential improvement: Create an undo_move() that allows you to keep changing the global board, and then reverse the changes if needed.

//     let mut temp_state = state.clone(); // independent copy
//     let cur_mask = 1 << cur_square;
//     let target_mask = 1 << target_square;

//     let temp_board = &mut temp_state.board;

//     // if capture
//     capture(target_mask, temp_board);

//     // add piece to target square
//     fill_square(cur_mask, target_mask, temp_board);

//      // remove piece from current square
//     empty_square(cur_mask, temp_board);


//     return temp_state;
// }



pub fn make_move(cur_square: i8, target_square: i8, state: &mut GameState, history: &mut History, stop_reset: bool) -> bool{

    // stop_reset is set to true during testing

    let cur_mask: u64 = 1<<cur_square;
    let target_mask: u64 = 1<<target_square;


    // is_legal checks if the player has a piece on cur_square that can be moved to target_square
    // is_legal does NOT account for self-check
    if !is_legal(cur_square, target_square, &state) {
        print!("{}", "Invalid move.\n");
        return false;
    }


    // save current state to history
    history.push(state.clone());



    let board = &mut state.board;


    // if capture
    // remove piece from opponent's piece
    capture(target_mask, board);

    // add piece to target square
    fill_square(cur_mask, target_mask, board);

     // remove piece from current square
    empty_square(cur_mask, board);


    en_passant(target_mask, state);
    update_en_passant_square(cur_square, target_square, state);

    castle(cur_square, target_square, state);
    update_castling_rights(cur_square, state);

    promotion(target_square, state);



    // toggle turn temporarily
    let side = state.side_to_move;
    let opponent = side.opposite();


    // Check self-check
    if is_check(state, opponent) {

        print!("SELFCHECK\n");
        // Undo move using history
        if let Some(prev) = history.pop() {
            *state = prev;
        }

        return false;
    }


    // toggle turns
    state.side_to_move = state.side_to_move.opposite(); 


    // CHECK FOR CHECKMATE LAST

    // Now it's the opponent's move, but first we check if they can make a move,
    // If not, it's either checkmate or stalemate.
    if is_checkmate_stalemate(state) {
        if is_check(state, state.side_to_move.opposite()){
            print!("CHECKMATE\n");
        } else {
            print!("STALEMATE\n");
        }

        if !stop_reset {
            state.reset(); // reset the game
        }
        return true;
    }


    return true;

}

pub fn capture(target_mask: u64, board: &mut Board) -> Option<i8>{

    /*
    
    1 → pawn, 2 → knight, 3 → bishop, 4 → rook, 5 → queen */

    if (target_mask & board.black_occupied) != 0 {

        board.black_occupied &= !target_mask;

        if board.black_pawns & target_mask != 0 {
            board.black_pawns &= !target_mask;
            return Some(1);
        }

        if board.black_knights & target_mask != 0 {
            board.black_knights &= !target_mask;
            return Some(2);
        }
        
        if board.black_bishops & target_mask != 0 {
            board.black_bishops &= !target_mask;
            return Some(3);
        }

        if board.black_rooks & target_mask != 0 {
            board.black_rooks &= !target_mask;
            return Some(4);
        }

        if board.black_queens & target_mask != 0 {
            board.black_queens &= !target_mask;
            return Some(5);
        }
       
    }
    if (target_mask & board.white_occupied) != 0 {
        board.white_occupied &= !target_mask;

        if board.white_pawns & target_mask != 0 {
            board.white_pawns &= !target_mask;
            return Some(1);
        }

        if board.white_knights & target_mask != 0 {
            board.white_knights &= !target_mask;
            return Some(2);
        }
        
        if board.white_bishops & target_mask != 0 {
            board.white_bishops &= !target_mask;
            return Some(3);
        }

        if board.white_rooks & target_mask != 0 {
            board.white_rooks &= !target_mask;
            return Some(4);
        }

        if board.white_queens & target_mask != 0 {
            board.white_queens &= !target_mask;
            return Some(5);
        }
    }

    None
}


pub fn fill_square(cur_mask: u64, target_mask: u64, board: &mut Board) {

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

pub fn empty_square(cur_mask: u64, board: &mut Board) {
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

    let promoted_pawns = match state.side_to_move {
        Color::White => target_mask & state.board.white_pawns & EIGHT_RANK,
        Color::Black => target_mask & state.board.black_pawns & FIRST_RANK,
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
        match state.side_to_move {
            Color::White => state.board.white_pawns &= !target_mask,
            Color::Black => state.board.black_pawns &= !target_mask,
        }

        // Add the promoted piece
        let valid = match promotion_piece.as_str() {
            "Q" => {
                match state.side_to_move {
                    Color::White => state.board.white_queens |= target_mask,
                    Color::Black =>state.board.black_queens |= target_mask,
                }
                true
            }
            "R" => {
                match state.side_to_move {
                    Color::White => state.board.white_rooks |= target_mask,
                    Color::Black => state.board.black_rooks |= target_mask,
                }
                true
            }
            "B" => {
                match state.side_to_move {
                    Color::White => state.board.white_bishops |= target_mask,
                    Color::Black => state.board.black_bishops |= target_mask,
                }
                true
            }
            "N" => {
                match state.side_to_move {
                    Color::White => state.board.white_knights |= target_mask,
                    Color::Black => state.board.black_knights |= target_mask,
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
    match state.side_to_move {
        Color::White => {
            // capture black's pawn "in front of" the target square
            state.board.black_pawns &= !(target_mask >> 8);
            state.board.black_occupied &= !(target_mask >> 8);
        }
        Color::Black => {
            // capture white's pawn "behind" the target square
            state.board.white_pawns &= !(target_mask << 8);
            state.board.white_occupied &= !(target_mask << 8);
        }
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

            // rook on a8 to d8
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

pub fn checked_squares(state: &GameState, side_checking: Color) -> u64 {

    /*
    Compute all the squares that player side_checking is targeting.
     */

    let board = &state.board;
    let mut all_targeted_squares: u64 = 0;

    match side_checking {

        Color::White => {

            all_targeted_squares |= pawn_moves(board.white_pawns, state, side_checking);
            all_targeted_squares |= knight_moves(board.white_knights, state, side_checking);
            all_targeted_squares |= bishop_moves(board.white_bishops, state, side_checking);
            all_targeted_squares |= rook_moves(board.white_rooks, state, side_checking);
            all_targeted_squares |= queen_moves(board.white_queens, state, side_checking);
            all_targeted_squares |= king_moves(board.white_king, state, side_checking, false);
        }

        Color::Black => {

            all_targeted_squares |= pawn_moves(board.black_pawns, state, side_checking);
            all_targeted_squares |= knight_moves(board.black_knights, state, side_checking);
            all_targeted_squares |= bishop_moves(board.black_bishops, state, side_checking);
            all_targeted_squares |= rook_moves(board.black_rooks, state, side_checking);
            all_targeted_squares |= queen_moves(board.black_queens, state, side_checking);
            all_targeted_squares |= king_moves(board.black_king, state, side_checking, false);

        }
    }

    return all_targeted_squares;


}

pub fn is_check(state: &GameState, side_checking: Color) -> bool { 

    /* compute ALL potential moves for side_checking
    Otherwise will not account for discovered checks!
    
    side_checking is the color of the player whose pieces we compute the checked_squares for,
    eg. if side_checking=Color::White, we want to check if any of white's pieces are checking
    black's king.
    */ 

    let board = &state.board;

    match side_checking {
        Color::White => return checked_squares(state, side_checking) & board.black_king != 0,
        Color::Black => return checked_squares(state, side_checking) & board.white_king != 0,
    }

}

pub fn is_checkmate_stalemate(state: &mut GameState) -> bool {

    let side = state.side_to_move;
    let opposite_side = side.opposite();

    // try all possible moves for the opponent
    // state.side_to_move is the opponent's color, eg. if white just made the move,
    // state.side_to_move = Color::Black.


    for cur_square in 0..64 {
        for target_square in 0..64 {

            if !is_legal(cur_square, target_square, state) {
                continue;
            }

            // Perform the move in-place and record captured piece
            let cur_mask = 1 << cur_square;
            let target_mask = 1 << target_square;

            // capture
            let piece_captured = capture(target_mask, &mut state.board);

            // move the piece
            fill_square(cur_mask, target_mask, &mut state.board);
            empty_square(cur_mask, &mut state.board);
            
            castle(cur_square, target_square, state);


            // Check if after this move, the side that just moved is NOT in check
            let in_check = is_check(state, opposite_side);

            // Undo the move
            undo_move(cur_square, target_square, state, opposite_side, piece_captured);

            // If at least one legal move exists that avoids check, it's NOT checkmate/stalemate
            if !in_check {
                return false;
            }
        }
    }

    // No escape moves found → checkmate or stalemate
    true
}



pub fn undo_move(original_square: i8, new_square: i8, state: &mut GameState, side: Color, piece_captured: Option<i8>) {

    let original_mask = 1<<original_square;
    let new_mask = 1<<new_square;

    // undo castling
    undo_castle(original_square, new_square, state);

    // fill the original square with the piece.
    fill_square(new_mask, original_mask, &mut state.board);

    // empty the new_square
    empty_square(new_mask, &mut state.board);

    // fill new square with captured piece
    restore_captured_piece(new_mask, state, side, piece_captured);

}

pub fn restore_captured_piece(new_mask: u64, state: &mut GameState, side: Color, piece_captured: Option<i8>) {

    let board = &mut state.board;

    match piece_captured {

        None => return,

        // pawn was captured
        Some(1) => {

            match side {

                Color::White => {
                    board.white_pawns |= new_mask;
                    board.white_occupied |= new_mask;
                }
                Color::Black => {
                    board.black_pawns |= new_mask;
                    board.black_occupied |= new_mask;
                }
            }
        }
        // knight was captured
        Some(2) => {
            match side {

                Color::White => {
                    board.white_knights |= new_mask;
                    board.white_occupied |= new_mask;
                }
                Color::Black => {
                    board.black_knights |= new_mask;
                    board.black_occupied |= new_mask;
                }
            }
        }
        // bishop was captured
        Some(3) => {
            match side {

                Color::White => {
                    board.white_bishops |= new_mask;
                    board.white_occupied |= new_mask;
                }
                Color::Black => {
                    board.black_bishops |= new_mask;
                    board.black_occupied |= new_mask;
                }
            }
        }

        // rook was captured
        Some(4) => {
            match side {

                Color::White => {
                    board.white_rooks |= new_mask;
                    board.white_occupied |= new_mask;
                }
                Color::Black => {
                    board.black_rooks |= new_mask;
                    board.black_occupied |= new_mask;
                }
            }
        }

        // queen was captured
        Some(5) => {
            match side {

                Color::White => {
                    board.white_queens |= new_mask;
                    board.white_occupied |= new_mask;
                }
                Color::Black => {
                    board.black_queens |= new_mask;
                    board.black_occupied |= new_mask;
                }
            }
        }
        Some(_) => return,
    }
}


pub fn undo_castle(original_square: i8, new_square: i8, state: &mut GameState) {

    let new_mask = 1<<new_square;

    let board = &mut state.board;


    if new_mask & board.white_king != 0 {

        if original_square == 4 && new_square == 6 { // kingside

            // move rook from f1 to h1
            board.white_rooks &= !(1<<5);
            board.white_rooks |= 1<<7;
        }

        if original_square == 4 && new_square == 2 { // queenside

            // move rook from d1 to a1
            board.white_rooks &= !(1<<3);
            board.white_rooks |= 1<<0;
        }
    }

    else if new_mask & board.black_king != 0 {

        if original_square == 60 && new_square == 62  {// king-side

            // rook on f8 to h8
            board.black_rooks &= !(1<<61);
            state.board.black_rooks |= 1<<63;
        }

        else if original_square == 60 && new_square == 58 { // queen-side

            // rook on d8 to a8
            state.board.black_rooks &= !(1<<59);
            state.board.black_rooks |= 1<<56;
        }
    }
}