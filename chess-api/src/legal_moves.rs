/*
For each piece, exists a function returning all targeted squares, input parameter is a bitboard representing
all squares occupied by the specific piece. 

- BOARD is a global instance of struct Board definied in bitboards.rs
- `BOARD.white_occupied` and `BOARD.black_occupied` track which squares are occupied by each color.
- Used to determine if a move is legal (cannot move to a square already occupied by own piece, allow moving to a square
occupied by opponent's piece (representing a capture)).

- MOVE is a global state variable indicating whose turn it is.
- 0 = white to move, 1 = black to move
- used to determine which pieces belong to opponent
*/

use crate::Board;

// Masking: https://www.chessprogramming.org/Square_Mapping_Considerations
const FILE_A: u64 = 0b100000001000000010000000100000001000000010000000100000001;
const FILE_H: u64 = 0b1000000010000000100000001000000010000000100000001000000010000000;
const FILE_B: u64 = 0b1000000010000000100000001000000010000000100000001000000010;
const FILE_G: u64 = 0b100000001000000010000000100000001000000010000000100000001000000;

const SECOND_RANK: u64 = 0b1111111100000000; 
const SEVENTH_RANK: u64 = 0xFF000000000000;
//const FIRST_RANK: u64 = 0x00000000000000FF;
//const EIGHT_RANK: u64 = 0xFF00000000000000;

pub fn knight_moves(pos: u64, board: &Board, to_move: u8) -> u64 { // masking inspo: https://www.chessprogramming.org/Knight_Pattern

    let mut targeted_squares: u64 = 0u64;

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



pub fn king_moves(pos: u64, board: &Board, to_move: u8) -> u64 { // add more checks later (for check, checkmate etc.)

    let mut targeted_squares: u64 = 0u64;

    let own_occupied = if to_move == 0 {
        board.white_occupied
    } else {
        board.black_occupied
    };


    targeted_squares |= (pos & !FILE_A) << 7 & !own_occupied; // up once, left once
    targeted_squares |= pos << 8 & !own_occupied; // one once
    targeted_squares |= (pos & !FILE_H) << 9 & !own_occupied; // up once, right once
    targeted_squares |= (pos & !FILE_A) >> 1 & !own_occupied;
    targeted_squares |= (pos & !FILE_H) << 1 & !own_occupied;
    targeted_squares |= (pos & !FILE_A) >> 9 & !own_occupied;
    targeted_squares |= pos >> 8 & !own_occupied;
    targeted_squares |= (pos & !FILE_H) >> 7 & !own_occupied;
    

    return targeted_squares;

}

pub fn rook_moves(pos: u64, board: &Board, to_move: u8) -> u64 {
    let mut targeted_squares = 0u64;
    let mut rooks = pos;

    while rooks != 0 {
        let square = rooks.trailing_zeros() as i8; // gives square of occupied bit, eg. 1110000.trailing_zeros() = 4.
        rooks &= rooks - 1; // removes least significant set bit

        targeted_squares |= helper_rook_moves(square, &board, to_move);
    }

    return targeted_squares;
}


pub fn helper_rook_moves(square: i8, board: &Board, to_move: u8) -> u64{

    let mut targeted_squares: u64 = 0u64;

    let unoccupied = !(board.white_occupied | board.black_occupied);


    // bitmasking 

    let cur_row = square/8;
    let cur_col = square % 8;
    let cur_mask = 1 << square;

    for n in 1..8-cur_row { // iterate upwards

        if (cur_mask << 8*n & unoccupied) != 0 {
            targeted_squares |= cur_mask << 8*n & unoccupied;
        }
        else if (cur_mask << 8*n & board.black_occupied) != 0 && (to_move == 0) {
            targeted_squares |= cur_mask << 8*n & board.black_occupied;
            break;
        }
        else if (cur_mask << 8*n & board.white_occupied) != 0 && (to_move == 1) {
            targeted_squares |= cur_mask << 8*n & board.white_occupied;
            break;
        }
        else {
            break;
        }
    }

    for n in 1..cur_row+2 { // iterate downwards

        if (cur_mask >> 8*n & unoccupied) != 0 {
            targeted_squares |= cur_mask >> 8*n & unoccupied;
        }
        else if (cur_mask >> 8*n & board.black_occupied) != 0 && (to_move == 0) {
            targeted_squares |= cur_mask >> 8*n & board.black_occupied;
            break;
        }
        else if (cur_mask >> 8*n & board.white_occupied) != 0 && (to_move == 1) {
            targeted_squares |= cur_mask >> 8*n & board.white_occupied;
            break;
        }
        else {
            break;
        }
    }

    for n in 1..8-cur_col { // iterate right

        let new_mask = (cur_mask << (n-1) & !FILE_H) << 1;

        if (new_mask & unoccupied) != 0 {
            targeted_squares |= new_mask & unoccupied;
        }

        else if to_move == 0 {
            targeted_squares |= new_mask & board.black_occupied;
            break;
        }
        else if to_move == 1 {
            targeted_squares |= new_mask & board.white_occupied;
            break;
        }
        else {
            break;
        }
    }

    for n in 1..8-cur_col+1 { // iterate left

        let new_mask = (cur_mask >> (n-1) & !FILE_A) >> 1;

        if (new_mask & unoccupied) != 0 {
            targeted_squares |= new_mask & unoccupied;
        }

        else if to_move == 0 {
            targeted_squares |= new_mask & board.black_occupied;
            break;
        }
        else if to_move == 1 {
            targeted_squares |= new_mask & board.white_occupied;
            break;
        }
        else {
            break;
        }
    }

    return targeted_squares;
        

}



pub fn bishop_moves(pos: u64, board: &Board, to_move: u8) -> u64 {
    let mut targeted_squares = 0u64;
    let mut bishops = pos;

    while bishops != 0 {
        let square = bishops.trailing_zeros() as i8; // gives square of occupied bit, eg. 1110000.trailing_zeros() = 4.
        bishops &= bishops - 1; // removes least significant set bit

        targeted_squares |= helper_bishop_moves(square, &board, to_move);
    }

    return targeted_squares;
}

pub fn helper_bishop_moves(square: i8, board: &Board, to_move: u8) -> u64 {


    let mut targeted_squares: u64 = 0u64;

    let unoccupied = !(board.white_occupied | board.black_occupied);

    // bitmasking 

    let cur_row = square/8;
    let cur_col = square % 8;
    let cur_mask = 1 << square;

    let max_up_right = 8-cur_row.min(8-cur_col); // limited by row and column
    let max_up_left = 8-cur_row.min(cur_col+1);

    let max_down_right = cur_row+1.min(8-cur_col);
    let max_down_left = cur_row+1.min(cur_col+1);


    for n in 1..max_up_right { // iterate up, right

        // 8*n+n = 9*n
        let new_mask = (cur_mask << 9*(n-1) & !FILE_H) << 9;

        if (new_mask & unoccupied) != 0 {
            targeted_squares |= new_mask & unoccupied;
        }
        else if to_move == 0 {
            targeted_squares |= new_mask & board.black_occupied;
            break;
        }
        else if to_move == 1 {
            targeted_squares |= new_mask & board.white_occupied;
            break;
        }
        else {
            break;
        }
    }

    for n in 1..max_up_left { // iterate up, left

        // 8*n - n = 7*n

        let new_mask = (cur_mask << 7*(n-1) & !FILE_A) << 7;

        if (new_mask & unoccupied) != 0 {
            targeted_squares |= new_mask & unoccupied;
        }
        else if to_move == 0 {
            targeted_squares |= new_mask & board.black_occupied;
            break;
        }
        else if to_move == 1 {
            targeted_squares |= new_mask & board.white_occupied;
        }
        else {
            break;
        }
    }

    for n in 1..max_down_right { // iteratre down, right

        //  7*n

        let new_mask = (cur_mask >> 7*(n-1) & !FILE_H) >> 7 ;

        if (new_mask & unoccupied) != 0 {
            targeted_squares |= new_mask & unoccupied;
        }
        else if to_move == 0 {
            targeted_squares |= new_mask & board.black_occupied;
            break;
        }
        else if to_move == 1 {
            targeted_squares |= new_mask & board.white_occupied;
        }
        else {
            break;
        }
    }

    for n in 1..max_down_left { // iterate down, left

        // 9*n

        let new_mask = (cur_mask >> 9*(n-1) & !FILE_A) >> 9;

        if (new_mask & unoccupied) != 0 {
            targeted_squares |= new_mask & unoccupied;
        }
        else if to_move == 0 {
            targeted_squares |= new_mask & board.black_occupied;
            break;
        }
        else if to_move == 1 {
            targeted_squares |= new_mask & board.white_occupied;
            break;
        }
        else {
            break;
        }
    }

    return targeted_squares;
}


pub fn queen_moves(square: u64, board: &Board, to_move: u8) -> u64 { // combine bishop&rook moves

    return rook_moves(square, board, to_move) | bishop_moves(square, board, to_move);

}


pub fn pawn_moves(pos: u64, board: &Board, to_move: u8) ->u64 {

    let mut targeted_squares: u64 = 0u64;

    let unoccupied = !(board.white_occupied | board.black_occupied);

    if to_move == 0 {

        let one_step = pos << 8 & unoccupied;
        let two_steps = (((SECOND_RANK << 8) & one_step) << 8) & unoccupied;
        let capture_left = board.black_occupied & ((pos&!FILE_A) << 7);
        let capture_right = board.black_occupied & ((pos&!FILE_H) << 9);

        targeted_squares |= one_step | two_steps | capture_left | capture_right;
     
    } else {

        let one_step = pos >> 8 & unoccupied;
        let two_steps = (((SEVENTH_RANK >> 8) & one_step) >> 8) & unoccupied;
        let capture_left = board.white_occupied & ((pos&!FILE_A) >> 9);
        let capture_right = board.white_occupied & ((pos&!FILE_H) >> 7);

        targeted_squares |= one_step | two_steps | capture_left | capture_right;
        
    };

    return targeted_squares;

}