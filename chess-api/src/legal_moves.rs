use crate::BOARD;
use crate::MOVE;

// prevent moving outside 8x8-board
pub fn is_valid_move(cur_square: i64, row_delta: i64, col_delta: i64) -> bool {

    if cur_square + 8*row_delta + col_delta < 0 || cur_square + 8*row_delta + col_delta > 63 {
        return false;
    }
    else if cur_square%8 + col_delta < 0 || cur_square%8 + col_delta > 7 {
        return false;
    }
    return true;
}

pub fn knight_moves(square: i64) -> u64 {

    let mut targeted_squares: u64 = 0u64;


    // row and column deltas
    let deltas: [(i64, i64); 8] = [
        (2, -1), // two rows up, one left
        (2, 1), 
        (1, -2),
        (1, 2),
        (-1, -2),
        (-1, 2),
        (-2, -1),
        (-2, 1)
    ];

    for (row_delta, col_delta) in deltas {
        if is_valid_move(square, row_delta, col_delta) {
            targeted_squares |= 1u64<<(square+8*row_delta+col_delta);
        }
    }

    return targeted_squares;
    
}



pub fn king_moves(square: i64) -> u64 { // add more checks later (for check, checkmate etc.)

    let mut targeted_squares: u64 = 0u64;

    let deltas: [(i64, i64); 8] = [
        (1, -1),
        (1, 0),
        (1, 1),
        (0, -1),
        (0, 1),
        (-1, -1),
        (-1, 0),
        (-1, 1)
    ];

    for (row_delta, col_delta) in deltas {
        if is_valid_move(square, row_delta, col_delta) {
            targeted_squares |= 1<<(square+row_delta*8+col_delta);

        }
    }

    return targeted_squares;


}


pub fn rook_moves(square: i64) -> u64{

    let board = BOARD.lock().unwrap();

    let mut targeted_squares: u64 = 0u64;

    let dirs: [(i64, i64); 4] = [
        (-1, 0), // down (bit 0 represents sqr a1)
        (1, 0), // up
        (0, -1), // left
        (0, 1) // right

    ];


    for (row_delta, col_delta) in dirs {
    
        let mut n = 1; // num steps in direction (row_delta, col_delta)

        while n < 8 {

            if is_valid_move(square, row_delta*n, col_delta*n) {
                // FIX: ADD MOVE DEPENDING ON MOVE/COLOR (if the piece can be captured, or if it's your own piece)
                if (board.black_occupied | board.white_occupied) & (1 << square+row_delta*8*n+col_delta*n) == 0 { // not occupied

                    targeted_squares |= 1 << square+row_delta*8*n+col_delta*n;
                }   
                else {
                    break;
                }
            }

            else {
                break;
            }

            n += 1;
        }
    }

    return targeted_squares;
        

}


pub fn bishop_moves(square: i64) -> u64 {

    let board = BOARD.lock().unwrap();

    let mut targeted_squares: u64 = 0u64;

    let dirs: [(i64, i64); 4] = [
        (1, -1), // up, left
        (1, 1),
        (-1, -1),
        (-1, 1)
    ];


    for (row_delta, col_delta) in dirs {

        let mut n = 1;

        while n < 8 {

            if is_valid_move(square, row_delta*n, col_delta*n) {

                if (board.white_occupied | board.black_occupied) & (1<<square+8*n*row_delta+n*col_delta) == 0 { // not occupied
                    targeted_squares |= 1<<square+8*n*row_delta+col_delta*n;
                }
                else {
                    break;
                }
            }
            else {
                break;
            }

            n += 1;
        }
    }


    return targeted_squares;
}


pub fn queen_moves(square: i64) -> u64 { // combine bishop&rook moves

    return rook_moves(square) | bishop_moves(square);

}


pub fn pawn_moves(square: i64) ->u64 {

    let mut targeted_squares: u64 = 0u64;
    let to_move = *MOVE.lock().unwrap(); // dereference ()
    let board = BOARD.lock().unwrap();

    if to_move == 0u8 { // white's move

        if 8 <= square && square <= 15 { // second rank --> allow two steps

            if is_valid_move(square, 2, 0) {
                
                if ((board.white_occupied | board.black_occupied) & 1<<square+8 == 0) && ((board.white_occupied | board.black_occupied) & 1<<square+16 == 0) {
                    targeted_squares |= 1<<square+16;
                }
            }
        }

        if is_valid_move(square, 1, 0) {

            if (board.white_occupied | board.black_occupied) & 1<<square+8 == 0 {
                targeted_squares |= 1<<square+8;
            }
        }
    }

    else if to_move == 1 { // black's move

        if 48 <= square && square <= 55 {

            if is_valid_move(square, -2, 0) {
                if ((board.white_occupied | board.black_occupied) & 1<<square-8 == 0) && ((board.white_occupied | board.black_occupied) & 1<<square-16 == 0) {
                    targeted_squares |= 1<<square-16;
                }
            }
        }
        
        if is_valid_move(square, -1, 0) {

            if (board.white_occupied | board.black_occupied) & 1<<square-8 == 0 {
                targeted_squares |= 1<<square-8;
            }

            }

    }

    return targeted_squares;
}