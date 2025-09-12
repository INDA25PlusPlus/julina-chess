pub mod bitboards; // multiple source files https://www.youtube.com/watch?v=lx5r7yzl1Ps

pub mod legal_moves;
pub mod perform_moves;

use once_cell::sync::Lazy;
use std::sync::Mutex;
use bitboards::Board;

 // GLOBAL STATES
pub static BOARD: Lazy<Mutex<Board>> = Lazy::new(|| {
    Mutex::new(Board::new())
});

pub static MOVE: Lazy<Mutex<u8>> = Lazy::new(|| { // 0: whites move, 1: blacks move
    Mutex::new(0)
});

#[allow(dead_code)]
fn game() {

    // take input in rust: https://users.rust-lang.org/t/how-to-get-user-input/5176/3
    use std::io::{stdin,stdout,Write};

    let mut s = String::new();
    let mut current = String::new();
    let mut target = String::new();


    loop {
        print!("Enter q to quit");

        let _=stdout().flush();
        stdin().read_line(&mut s).expect("Did not enter a correct string");

        if let Some('\n')=s.chars().next_back() {
            s.pop();
        }
        if let Some('\r')=s.chars().next_back() {
            s.pop();
        }

        if s == "q".to_string() {
            break;
        } else {
            print!("Enter square from where to move");
            current.clear();
            stdin().read_line(&mut current).expect("Did not enter a correct string");
            

            print!("Enter square to where to move");
            target.clear();
            stdin().read_line(&mut target).expect("Did not enter a correct string");


        }
        println!("You typed: {}",s);


        s.clear();


    }

}



     

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
    print!("{}", "\n\n");
}



#[cfg(test)] // kompileras endast när tester körs
mod tests {

    use super::*;
   // use crate::{game};


    #[test]
    fn test_pawn_moves() {

        let board = BOARD.lock().unwrap();

        let result = legal_moves::pawn_moves(1<<8, &board, 0);
        assert_eq!(result, 1<<16 | 1<<24);
        // let result = legal_moves::pawn_moves(1<<60) ;
        // assert_eq!(result, 0);
        //dbg_print_board(legal_moves::pawn_moves(1<<8 | 1<<9, &board, 0));
    }

    #[test]
    fn test_king_moves() {

        let board = BOARD.lock().unwrap();

        legal_moves::king_moves(1<<30, &board, 0);
        //dbg_print_board(legal_moves::king_moves(30));
    }

    #[test]
    fn test_knight_moves() {

        let board = BOARD.lock().unwrap();

        legal_moves::knight_moves(1<<16, &board, 0);
    }

    #[test]
    fn test_rook_moves() {

        let board = BOARD.lock().unwrap();
        legal_moves::rook_moves(1<<16, &board, 0);
        // dbg_print_board(legal_moves::rook_moves(1<<24 | 1<<34));
        // dbg_print_board(legal_moves::helper_rook_moves(24));

    }

    #[test]
    fn test_bishop_moves() {
        let board = BOARD.lock().unwrap();
        legal_moves::bishop_moves(1<<0, &board, 0);
        legal_moves::bishop_moves(1<<8, &board, 0);
        legal_moves::bishop_moves(1<<63, &board, 0);
        // dbg_print_board(legal_moves::bishop_moves(1<<30));
        // dbg_print_board(legal_moves::bishop_moves(1<<28));
    }

    #[test]
    fn test_queen_moves() {

        let board = BOARD.lock().unwrap();

        legal_moves::queen_moves(16, &board, 0);
    }

    #[test]
    fn test_print_board(){

        let board = Board::new(); // start configuration

        bitboards::print_board(&board); // print start config
    }

    #[test]
    fn test_is_legal() {

        let board = Board::new(); // start configuration
        
        let mut result = perform_moves::is_legal(12, 28, &board, 0); //legal
        assert_eq!(result, true);
        result = perform_moves::is_legal(50, 40, &board, 0); // not legal
        assert_eq!(result, false);
    }

    #[test]
    fn test_is_check() {

        let board = Board::new(); // start configuration

        assert_eq!(perform_moves::is_check(&board, 0), false); 
    }



    #[test]
    fn test_make_move() {

        perform_moves::make_move(12, 28); // e2-e4
    
        perform_moves::make_move(52, 36); // e7-e5

        perform_moves::make_move(5, 26); //bf1-c4

        perform_moves::make_move(51, 43); // d7-d6

        perform_moves::make_move(26, 53); //bc4-f7+


        let board = BOARD.lock().unwrap();

        assert_eq!(perform_moves::is_check(&board, 0), true);

        // print board
        bitboards::print_board(&board);
    }


    // #[test]
    // fn test_make_move() {
    //     //perform_moves::is_legal(8, 16); // legal
    //     perform_moves::make_move(8, 24);

    //     // print board
    //     bitboards::print_board();


    //     perform_moves::make_move(24, 40); // not legal

    //     bitboards::print_board();

    //     perform_moves::make_move(48, 40);

    //     let board = BOARD.lock().unwrap();
    //     dbg_print_board(board.white_pawns);
    //     drop(board);
    //     dbg_print_board(legal_moves::pawn_moves(24));


    //     // print board
    //     bitboards::print_board();
    // }

    // #[test]
    // fn test_game_take_input() {
    //     game();
    // }

    
}
