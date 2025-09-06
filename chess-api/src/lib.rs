pub mod bitboards; // multiple source files https://www.youtube.com/watch?v=lx5r7yzl1Ps


#[cfg(test)]
mod tests {
    // use super::*;
    use crate::bitboards;

    #[test]
    fn test_print_board(){

        bitboards::print_board();
    }

}
