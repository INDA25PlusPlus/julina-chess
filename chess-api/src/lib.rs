pub mod bitboards; // multiple source files https://www.youtube.com/watch?v=lx5r7yzl1Ps

pub mod legal_moves;

#[cfg(test)]
mod tests {
    // use super::*;
    use crate::{bitboards, legal_moves};
    

    #[test]
    fn test_print_board(){

        bitboards::print_board();
    }

    #[test]
    fn test_knight_moves(){

        print!("{:b}", legal_moves::knight_moves(0));
        print!("{}", "\n");
    }

}
