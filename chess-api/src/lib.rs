pub mod bitboards; // multiple source files https://www.youtube.com/watch?v=lx5r7yzl1Ps

pub mod legal_moves;

use once_cell::sync::Lazy;
use std::sync::Mutex;
use bitboards::Board;

// GLOBAL STATES
pub static BOARD: Lazy<Mutex<Board>> = Lazy::new(|| {
    Mutex::new(Board::new())
});



#[allow(dead_code)] // (only used in tests)
fn dbg_print_board(bb: u64) { // for debugging and testing

     // print the board
    /* The first square to be printed should be a8, so we get
    a8 b8 ... h8
    a7 b7 ... h7*/ 


    for i in (0..8).rev() { // print top to down

        for j in 0..8 { // print left to right
            print!("{} ", (bb >> i*8+j) & 1);
        }
        print!("{}", "\n\n");
    

    }
}



#[cfg(test)]
mod tests {
    // use super::*;
    use crate::{bitboards, dbg_print_board, legal_moves};
    

    #[test]
    fn test_print_board(){

        bitboards::print_start_configuration();
    }


    #[test]
    fn test_bishop_moves() {

        print!("{:b}", legal_moves::bishop_moves(0));
        print!("{}", "\n");
        dbg_print_board(legal_moves::bishop_moves(8));
        print!("{}", "\n");
    }

}
