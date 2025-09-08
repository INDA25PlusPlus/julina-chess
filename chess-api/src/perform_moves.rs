
use crate::legal_moves::bishop_moves;
use crate::legal_moves::king_moves;
use crate::legal_moves::knight_moves;
use crate::legal_moves::pawn_moves;
use crate::legal_moves::queen_moves;
use crate::legal_moves::rook_moves;
use crate::BOARD;
use crate::MOVE;



pub fn is_legal(cur_square: i64, target_square: i64) -> bool {

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


pub fn read_move(cur_square: String, target_square: String) {

    // convert string to bit (and check if invalid cur_square or target_square)


}


pub fn make_move(cur_square: u8, target_square: u8) {

}