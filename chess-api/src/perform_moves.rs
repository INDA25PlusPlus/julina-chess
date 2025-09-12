
use crate::legal_moves::bishop_moves;
use crate::legal_moves::king_moves;
use crate::legal_moves::knight_moves;
use crate::legal_moves::pawn_moves;
use crate::legal_moves::queen_moves;
use crate::legal_moves::rook_moves;
use crate::Board;
use crate::BOARD;
use crate::MOVE;



pub fn is_legal(cur_square: i8, target_square: i8, board: &Board, to_move: u8) -> bool {

    if cur_square < 0 || cur_square > 63 || target_square < 0 || target_square > 63 {
        return false;
    }

    let cur_mask: u64 = 1<<cur_square;
    let target_mask: u64 = 1<<target_square;

    // https://users.rust-lang.org/t/mutex-lock-twice/88414
    // can't lock Mutex BOARD twice, but since it's also locked in pawn_moves(), bishop_moves() etc.
    // the lock needs to be dropped before calling these functions
    // https://users.rust-lang.org/t/when-is-drop-called-on-mutex/6571/2

    if ((cur_mask & board.white_pawns) != 0) && to_move == 0 {
        return (pawn_moves(cur_mask, &board, to_move) & target_mask) != 0;
    }

    if ((cur_mask & board.white_bishops) != 0) && to_move == 0 {
        return (bishop_moves(cur_mask, &board, to_move) & target_mask) != 0;
    }

    if ((cur_mask & board.white_knights) != 0) && to_move == 0 {
        return (knight_moves(cur_mask, &board, to_move) & target_mask) != 0;
    }

    if ((cur_mask & board.white_rooks) != 0) && to_move == 0 {
        return (rook_moves(cur_mask, &board, to_move) & target_mask) != 0;
    }

    if ((cur_mask & board.white_queens) != 0) && to_move == 0 {
        return (queen_moves(cur_mask, &board, to_move) & target_mask) != 0;
    }

    if ((cur_mask & board.white_king) != 0) && to_move == 0 {
        return (king_moves(cur_mask, &board, to_move) & target_mask) != 0;
    }

 
    if ((cur_mask & board.black_pawns) != 0) && to_move == 1 {
        return (pawn_moves(cur_mask, &board, to_move) & target_mask) != 0;
    }

    if ((cur_mask & board.black_bishops) != 0) && to_move == 1 {
        return (bishop_moves(cur_mask, &board, to_move) & target_mask) != 0;
    }

    if ((cur_mask & board.black_knights) != 0) && to_move == 1 {
        return (knight_moves(cur_mask, &board, to_move) & target_mask) != 0;
    }

    if ((cur_mask & board.black_rooks) != 0) && to_move == 1 {
        return (rook_moves(cur_mask, &board, to_move) & target_mask) != 0;
    }

    if ((cur_mask & board.black_queens) != 0) && to_move == 1 {
        return (queen_moves(cur_mask, &board, to_move) & target_mask) != 0;
    }

    if ((cur_mask & board.black_king) != 0) && to_move == 1 {
        return (king_moves(cur_mask, &board, to_move) & target_mask) != 0;
    }


    return false;
}


pub fn simulate_make_move(cur_square: i8, target_square: i8, board: &Board) -> Board{ 
    // doesn't actually perform the move, just "pretends" to make the move -> for checks of check, checkmate etc.

    // this is slow and requires to make tons of clones, BUT easier to implement for now.
    // Potential improvement: Create an undo_move() that allows you to keep changing the global board, and then reverse the changes if needed.

    let mut temp_board = board.clone(); // independent copy
    let cur_mask = 1 << cur_square;
    let target_mask = 1 << target_square;

    // if capture
    if (target_mask & temp_board.black_occupied) != 0 {
        temp_board.black_occupied &= !target_mask;
        temp_board.black_pawns &= !target_mask;
        temp_board.black_king &= !target_mask;
        temp_board.black_queens &= !target_mask;
        temp_board.black_knights &= !target_mask;
        temp_board.black_rooks &= !target_mask;
        temp_board.black_bishops &= !target_mask;
    }
    if (target_mask & temp_board.white_occupied) != 0 {
        temp_board.white_occupied &= !target_mask;
        temp_board.white_pawns &= !target_mask;
        temp_board.white_king &= !target_mask;
        temp_board.white_queens &= !target_mask;
        temp_board.white_knights &= !target_mask;
        temp_board.white_rooks &= !target_mask;
        temp_board.white_bishops &= !target_mask;
    }


    // add piece to target square
    if (cur_mask & temp_board.white_pawns) != 0 {
        temp_board.white_occupied |= target_mask;
        temp_board.white_pawns |= target_mask;
    }

    if (cur_mask & board.white_king) != 0 {
        temp_board.white_occupied |= target_mask;
        temp_board.white_king |= target_mask;
    }

    if (cur_mask & temp_board.white_queens) != 0 {
        temp_board.white_occupied |= target_mask;
        temp_board.white_queens |= target_mask;
    }

    if (cur_mask & temp_board.white_knights) != 0 {
        temp_board.white_occupied |= target_mask;
        temp_board.white_knights |= target_mask;
    }

    if (cur_mask & temp_board.white_rooks) != 0 {
        temp_board.white_occupied |= target_mask;
        temp_board.white_rooks |= target_mask;
    }

    if (cur_mask & temp_board.white_bishops) != 0 {
        temp_board.white_occupied |= target_mask;
        temp_board.white_bishops |= target_mask;
    }

    if (cur_mask & temp_board.black_pawns) != 0 {
        temp_board.black_occupied |= target_mask;
        temp_board.black_pawns |= target_mask;
    }

    if (cur_mask & temp_board.black_king) != 0 {
        temp_board.black_occupied |= target_mask;
        temp_board.black_king |= target_mask;
    }

    if (cur_mask & temp_board.black_queens) != 0 {
        temp_board.black_occupied |= target_mask;
        temp_board.black_queens |= target_mask;
    }

    if (cur_mask & temp_board.black_knights) != 0 {
        temp_board.black_occupied |= target_mask;
        temp_board.black_knights |= target_mask;
    }

    if (cur_mask & temp_board.black_rooks) != 0 {
        temp_board.black_occupied |= target_mask;
        temp_board.black_rooks |= target_mask;
    }

    if (cur_mask & temp_board.black_bishops) != 0 {
        temp_board.black_occupied |= target_mask;
        temp_board.black_bishops |= target_mask;
    }

     // remove piece from current square
    temp_board.white_occupied &= !cur_mask;
    temp_board.white_pawns &= !cur_mask;
    temp_board.white_king &= !cur_mask;
    temp_board.white_queens &= !cur_mask;
    temp_board.white_knights &= !cur_mask;
    temp_board.white_rooks &= !cur_mask;
    temp_board.white_bishops &= !cur_mask;

    temp_board.black_occupied &= !cur_mask;
    temp_board.black_pawns &= !cur_mask;
    temp_board.black_king &= !cur_mask;
    temp_board.black_queens &= !cur_mask;
    temp_board.black_knights &= !cur_mask;
    temp_board.black_rooks &= !cur_mask;
    temp_board.black_bishops &= !cur_mask;


    return temp_board;
}



pub fn make_move(cur_square: i8, target_square: i8) {

    let mut board = BOARD.lock().unwrap();
    let mut to_move = MOVE.lock().unwrap(); 
    let cur_mask: u64 = 1<<cur_square;
    let target_mask: u64 = 1<<target_square;

    if !is_legal(cur_square, target_square, &board, *to_move) {
        print!("{}", "Invalid move.\n");
        return;
    }


    // if capture
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


    // add piece to target square
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

     // remove piece from current square
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


    if is_check(&board, *to_move) {
        print!("Check!\n");
    }

    // toggle turns
    *to_move = (*to_move+1)%2;

    return;

}


pub fn checked_squares(board: &Board, to_move: u8) -> u64 {


    if to_move == 0 {

        let mut all_targeted_squares: u64 = 0;

        all_targeted_squares |= pawn_moves(board.white_pawns, &board, to_move);
        all_targeted_squares |= knight_moves(board.white_knights, &board, to_move);
        all_targeted_squares |= bishop_moves(board.white_bishops, &board, to_move);
        all_targeted_squares |= rook_moves(board.white_rooks, &board, to_move);
        all_targeted_squares |= queen_moves(board.white_queens, &board, to_move);
        all_targeted_squares |= king_moves(board.white_king, &board, to_move);

        return all_targeted_squares;
    
    } else {

        let mut all_targeted_squares: u64 = 0;

        all_targeted_squares |= pawn_moves(board.black_pawns, &board, to_move);
        all_targeted_squares |= knight_moves(board.black_knights, &board, to_move);
        all_targeted_squares |= bishop_moves(board.black_bishops, &board, to_move);
        all_targeted_squares |= rook_moves(board.black_rooks, &board, to_move);
        all_targeted_squares |= queen_moves(board.black_queens, &board, to_move);
        all_targeted_squares |= king_moves(board.black_king, &board, to_move);

        return all_targeted_squares;

    }


}


pub fn is_check(board: &Board, to_move:u8) -> bool { // call before changing moves


    /* compute ALL potential moves for the color that just moved.
    Otherwise will not account for discovered checks!*/ 

    if to_move == 0 {
        return checked_squares(board, to_move) & board.black_king != 0;

    } else {
        return checked_squares(board, to_move) & board.white_king != 0;
    }

}


pub fn is_checkmate(board: &Board, to_move: u8) -> bool {


    






    return false;
}