
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
    let cur_mask: u64 = 1<<cur_square;
    let target_mask: u64 = 1<<target_square;

    // https://users.rust-lang.org/t/mutex-lock-twice/88414
    // can't lock Mutex BOARD twice, but since it's also locked in pawn_moves(), bishop_moves() etc.
    // the lock needs to be dropped before calling these functions
    // https://users.rust-lang.org/t/when-is-drop-called-on-mutex/6571/2
    if (cur_mask & (board.white_pawns | board.black_pawns)) != 0 {
        drop(board);
        return (pawn_moves(cur_square) & target_mask) != 0;
    }
    if (cur_mask & (board.white_bishops | board.black_bishops)) != 0 {
        drop(board);
        return (bishop_moves(cur_square) & target_mask) != 0;
    }
    if (cur_mask & (board.white_king | board.black_king)) != 0 {
        drop(board);
        return (king_moves(cur_square) & target_mask) != 0;
    }
    if (cur_mask & (board.white_knights | board.black_knights)) != 0 {
        drop(board);
        return (knight_moves(cur_square) & 1<< target_square) != 0;
    }
    if (cur_mask & (board.white_rooks | board.black_rooks)) != 0 {
        drop(board);
        return (rook_moves(cur_square) & 1 <<target_square) != 0;
    }
    if (cur_mask & (board.white_queens | board.black_queens)) != 0 {
        drop(board);
        return (queen_moves(cur_square) & target_mask) != 0;
    }

    return false;
}


// pub fn read_move(cur_square: String, target_square: String) {

//     // convert string to bit (and check if invalid cur_square or target_square)



// }


pub fn make_move(cur_square: i8, target_square: i8) {

    if !is_legal(cur_square, target_square) {
        print!("{}", "Invalid move.");
        return;
    }

    let mut board = BOARD.lock().unwrap();
    let mut to_move = MOVE.lock().unwrap();
    let cur_mask: u64 = 1<<cur_square;
    let target_mask: u64 = 1<<target_square;


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

    // toggle turns
    *to_move = (*to_move+1)%2;

    return;

}