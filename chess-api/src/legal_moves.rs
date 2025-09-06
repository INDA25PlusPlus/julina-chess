

// prevent moving outside 8x8-board
pub fn valid_move(cur_square: i64, row_delta: i64, col_delta: i64) -> bool {

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
        if valid_move(square, row_delta, col_delta) {
            print!("{} ", square);
            print!("{} ", row_delta);
            print!("{}", col_delta);
            print!("{}", "\n");
            targeted_squares |= 1u64<<(square+8*row_delta+col_delta);
        }
    }

    return targeted_squares;
    
}