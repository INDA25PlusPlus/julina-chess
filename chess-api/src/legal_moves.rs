/*
For each piece, exists a function returning targeted squares from a square (passed in as parameter).

- is_valid_move() checks that a target square is within board bounds and prevents overflow etc. when performing bit shifts

- BOARD is a global instance of struct Board definied in bitboards.rs
- `BOARD.white_occupied` and `BOARD.black_occupied` track which squares are occupied by each color.
- Used to determine if a move is legal (cannot move to a square already occupied by own piece, allow moving to a square
occupied by opponent's piece (representing a capture)).

- MOVE is a global state variable indicating whose turn it is.
- 0 = white to move, 1 = black to move
- used to determine which pieces belong to opponent
*/



use crate::BOARD;
use crate::MOVE;

// Masking: https://www.chessprogramming.org/Square_Mapping_Considerations
const FILE_A: u64 = 0b100000001000000010000000100000001000000010000000100000001;
const FILE_H: u64 = 0b1000000010000000100000001000000010000000100000001000000010000000;
const FILE_B: u64 = 0b1000000010000000100000001000000010000000100000001000000010;
const FILE_G: u64 = 0b100000001000000010000000100000001000000010000000100000001000000;

pub fn is_valid_move(cur_square: i64, row_delta: i64, col_delta: i64) -> bool { // prevent moving outside 8x8-board

    if cur_square + 8*row_delta + col_delta < 0 || cur_square + 8*row_delta + col_delta > 63 {
        return false;
    }
    else if cur_square%8 + col_delta < 0 || cur_square%8 + col_delta > 7 {
        return false;
    }
    return true;
}

pub fn knight_moves(square: i64) -> u64 { // masking inspo: https://www.chessprogramming.org/Knight_Pattern

    let mut targeted_squares: u64 = 0u64;
    let board = BOARD.lock().unwrap();
    let to_move = *MOVE.lock().unwrap();

    let pos = 1 << square; // knight bitboard

    let not_a_file = !FILE_A;
    let not_ab_file = !(FILE_A | FILE_B);
    let not_h_file = !FILE_H;
    let not_gh_file = !(FILE_G | FILE_H);

    targeted_squares |= (pos & not_h_file) << 17; // up twice, right
    targeted_squares |= (pos & not_gh_file) << 10; // up once, right twice
    targeted_squares |= (pos & not_gh_file) >> 6; // down once, right twice
    targeted_squares |= (pos & not_h_file) >> 15;
    targeted_squares |= (pos & not_a_file) << 15;
    targeted_squares |= (pos & not_ab_file) << 6;
    targeted_squares |= (pos & not_ab_file) >> 10;
    targeted_squares |= (pos & not_a_file) >> 17;


    if to_move == 0 {
        targeted_squares &= !board.white_occupied;
    } else {
        targeted_squares &= !board.black_occupied;
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
    let to_move = *MOVE.lock().unwrap();

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

                let new_square = square+row_delta*8*n+col_delta*n;
        
                if (board.black_occupied | board.white_occupied) & (1 << new_square) == 0 { // not occupied

                    targeted_squares |= 1 << new_square;
                }  
                
                // If the occupied square is of the opponent's color, add it to targeted squares
                else if to_move == 0 && (board.black_occupied & (1 << square+row_delta*8*n+col_delta*n)) != 0 { 
                   targeted_squares |= 1 << new_square;
                   break;
                }

                else if to_move == 1 && (board.white_occupied & (1 << square+row_delta*8*n+col_delta*n)) != 0 { 
                   targeted_squares |= 1 << new_square;
                   break;
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
    let to_move = *MOVE.lock().unwrap();

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

                let new_square: i32 = 1<<square+8*n*row_delta+n*col_delta;

                
                if (board.white_occupied | board.black_occupied) & (1<<new_square) == 0 { // not occupied
                    targeted_squares |= 1<<new_square;
                }

                // If the occupied square is of the opponent's color, add it to targeted squares
                else if to_move == 0 && (board.black_occupied & (1<<new_square) != 0) { // white to move, can caputre black's piece
                    targeted_squares |= 1<<new_square;
                    break;
                }

                else if to_move == 1 && (board.white_occupied & (1<<new_square) != 0) {
                    targeted_squares |= 1<<new_square;
                    break;
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

    // conditionally assigning: https://stackoverflow.com/questions/76794659/conditionally-assigning-to-a-variable-using-if
    let (step, lo, hi, opponent_occupied) = if to_move == 0 { // white to move
            (1, 8, 15, board.black_occupied)
        } else { // blakc to move
            (-1, 48, 55, board.white_occupied)
        };


    // two steps
    if lo <= square && square <= hi { // on second(seventh) rank, allow two steps

        if is_valid_move(square, step*2, 0) {
            if ((board.white_occupied | board.black_occupied) & 1<<square+8*step == 0) && ((board.white_occupied | board.black_occupied) & 1<<square+16*step == 0) {
                    targeted_squares |= 1<<square+16*step;
                }
        } 
    }
    // one step
    if is_valid_move(square, step, 0) {

        if (board.white_occupied | board.black_occupied) & 1<<square+8*step == 0 {
            targeted_squares |= 1<<square+8*step;
        }
    }

    // captures
    // left
    if is_valid_move(square, step, -1) {

        if opponent_occupied & 1<<square+8*step-1 != 0 {
            targeted_squares |= 1<<square+8*step-1;
        }
    }
    // right
    if is_valid_move(square, step, 1) {
        if opponent_occupied & 1<<square+8*step+1 != 0 {
            targeted_squares |= 1<<square+8*step+1;
        }
    }

    return targeted_squares;

}