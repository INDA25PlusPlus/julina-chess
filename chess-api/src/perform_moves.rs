
use crate::legal_moves::bishop_moves;
use crate::legal_moves::king_moves;
use crate::legal_moves::knight_moves;
use crate::legal_moves::pawn_moves;
use crate::legal_moves::queen_moves;
use crate::legal_moves::rook_moves;
use crate::BOARD;
use crate::MOVE;



pub fn is_legal(cur_square: i8, target_square: i8) -> bool {

    if cur_square < 0 || cur_square > 63 || target_square < 0 || target_square > 63 {
        return false;
    }

    let board = BOARD.lock().unwrap();

    if 1 << cur_square & (board.white_pawns | board.black_pawns) != 0 {
        return pawn_moves(cur_square) & 1 << target_square != 0;
    }
    if 1 << cur_square & (board.white_bishops | board.black_bishops) != 0 {
        return bishop_moves(cur_square) & 1 << target_square != 0;
    }
    if 1 << cur_square & (board.white_king | board.black_king) != 0 {
        return king_moves(cur_square) & 1 << target_square != 0;
    }
    if 1 << cur_square & (board.white_knights | board.black_knights) != 0 {
        return knight_moves(cur_square) & 1<< target_square != 0;
    }
    if 1 << cur_square & (board.white_rooks | board.black_rooks) != 0 {
        return rook_moves(cur_square) & 1 <<target_square != 0;
    }
    if 1 << cur_square & (board.white_queens | board.black_queens) != 0 {
        return queen_moves(cur_square) & 1 << target_square != 0;
    }

    return false;
}


// pub fn read_move(cur_square: String, target_square: String) {

//     // convert string to bit (and check if invalid cur_square or target_square)




// }


pub fn make_move(cur_square: i8, target_square: i8) {

    let mut board = BOARD.lock().unwrap();
    let mut to_move = MOVE.lock().unwrap();

    if !is_legal(cur_square, target_square) {
        print!("{}", "Invalid move.");
        return;
    }

    let cur_mask: u64 = 1<<cur_square;
    let target_mask: u64 = 1<<target_square;


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

    // add piece to target square
    if target_mask & board.white_pawns != 0 {
        board.white_occupied |= target_mask;
        board.white_pawns |= target_mask;
    }

    if target_mask & board.white_king != 0 {
        board.white_occupied |= target_mask;
        board.white_king |= target_mask;
    }

    if target_mask & board.white_queens != 0 {
        board.white_occupied |= target_mask;
        board.white_queens |= target_mask;
    }

    if target_mask & board.white_knights != 0 {
        board.white_occupied |= target_mask;
        board.white_knights |= target_mask;
    }

    if target_mask & board.white_rooks != 0 {
        board.white_occupied |= target_mask;
        board.white_rooks |= target_mask;
    }

    if target_mask & board.white_bishops != 0 {
        board.white_occupied |= target_mask;
        board.white_bishops |= target_mask;
    }

    if target_mask & board.black_pawns != 0 {
        board.black_occupied |= target_mask;
        board.black_pawns |= target_mask;
    }

    if target_mask & board.black_king != 0 {
        board.black_occupied |= target_mask;
        board.black_king |= target_mask;
    }

    if target_mask & board.black_queens != 0 {
        board.black_occupied |= target_mask;
        board.black_queens |= target_mask;
    }

    if target_mask & board.black_knights != 0 {
        board.black_occupied |= target_mask;
        board.black_knights |= target_mask;
    }

    if target_mask & board.black_rooks != 0 {
        board.black_occupied |= target_mask;
        board.black_rooks |= target_mask;
    }

    if target_mask & board.black_bishops != 0 {
        board.black_occupied |= target_mask;
        board.black_bishops |= target_mask;
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

    // toggle turns
    *to_move = (*to_move+1)%2;

}