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

const SECOND_RANK: u64 = 0b1111111100000000; 
const SEVENTH_RANK: u64 = 0xFF000000000000;

pub fn is_valid_move(cur_square: i8, row_delta: i8, col_delta: i8) -> bool { // prevent moving outside 8x8-board

    if cur_square + 8*row_delta + col_delta < 0 || cur_square + 8*row_delta + col_delta > 63 {
        return false;
    }
    else if cur_square%8 + col_delta < 0 || cur_square%8 + col_delta > 7 {
        return false;
    }
    return true;
}

pub fn knight_moves(square: i8) -> u64 { // masking inspo: https://www.chessprogramming.org/Knight_Pattern

    let mut targeted_squares: u64 = 0u64;
    let board = BOARD.lock().unwrap();
    let to_move = *MOVE.lock().unwrap();

    let pos = 1 << square; // knight bitboard

    let not_a_file = !FILE_A;
    let not_ab_file = !(FILE_A | FILE_B);
    let not_h_file = !FILE_H;
    let not_gh_file = !(FILE_G | FILE_H);

    targeted_squares |= (pos & not_h_file) << 17; // up twice, right once
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



pub fn king_moves(square: i8) -> u64 { // add more checks later (for check, checkmate etc.)

    let mut targeted_squares: u64 = 0u64;
    let pos = 1 << square;

    targeted_squares |= (pos & !FILE_A) << 7; // up once, left once
    targeted_squares |= pos << 8; // one once
    targeted_squares |= (pos & !FILE_H) << 9; // up once, right once
    targeted_squares |= (pos & !FILE_A) >> 1;
    targeted_squares |= (pos & !FILE_H) << 1;
    targeted_squares |= (pos & !FILE_A) >> 9;
    targeted_squares |= pos >> 8;
    targeted_squares |= (pos & !FILE_H) >> 7;

    return targeted_squares;

}


pub fn rook_moves(square: i8) -> u64{

    let board = BOARD.lock().unwrap();
    let to_move = *MOVE.lock().unwrap();

    let mut targeted_squares: u64 = 0u64;

    let dirs: [(i8, i8); 4] = [
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
                else if to_move == 0 { 
                   targeted_squares |= board.black_occupied & (1 << new_square);
                   break;
                } else if to_move == 1 { 
                   targeted_squares |= board.white_occupied & (1 << new_square);
                   break;
                } else {
                    break;
                }
            } else {
                break;
            }

            n += 1;
        }
    }

    return targeted_squares;
        

}


pub fn bishop_moves(square: i8) -> u64 {

    let board = BOARD.lock().unwrap();
    let to_move = *MOVE.lock().unwrap();

    let mut targeted_squares: u64 = 0u64;

    let dirs: [(i8, i8); 4] = [
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
                else if to_move == 0 { // white to move, can caputre black's piece
                    targeted_squares |= board.black_occupied & (1<<new_square);
                    break;
                } else if to_move == 1 {
                    targeted_squares |= board.white_occupied & (1<<new_square);
                    break;
                } else {
                    break;
                }
            } else {
                break;
            }

            n += 1;
        }
    }
    return targeted_squares;
}


pub fn queen_moves(square: i8) -> u64 { // combine bishop&rook moves

    return rook_moves(square) | bishop_moves(square);

}


pub fn pawn_moves(square: i8) ->u64 {

    let mut targeted_squares: u64 = 0u64;
    let to_move = *MOVE.lock().unwrap(); // dereference ()
    let board = BOARD.lock().unwrap();

    let pos: u64 = 1u64 << square;
    let unoccupied = !(board.white_occupied | board.black_occupied);


    if to_move == 0 {

        let one_step = pos << 8 & unoccupied;
        let two_steps = (((SECOND_RANK << 8) & one_step) << 8) & unoccupied;
        let capture_left = board.black_occupied & ((pos&!FILE_A) << 8 - 1);
        let capture_right = board.black_occupied & ((pos&!FILE_H) << 8 + 1);

        targeted_squares |= one_step | two_steps | capture_left | capture_right;
     
    } else {

        let one_step = pos >> 8 & unoccupied;
        let two_steps = (((SEVENTH_RANK >> 8) & one_step) >> 8) & unoccupied;
        let capture_left = board.black_occupied & ((pos&!FILE_A) >> 8 - 1);
        let capture_right = board.black_occupied & ((pos&!FILE_H) >> 8 + 1);

        targeted_squares |= one_step | two_steps | capture_left | capture_right;
        
    };

    return targeted_squares;

}