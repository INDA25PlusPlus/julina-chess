
use crate::bitboards::Board;


// FOLLOWING CODE ONLY USED TO PRINT (VISUALIZE) THE BOARD

pub fn print_board(board: &Board) {

    use std::collections::HashMap;

    let boards: [u64; 12] = [board.white_pawns, board.white_rooks, board.white_knights, board.white_bishops, board.white_queens, board.white_king,
                                board.black_pawns, board.black_rooks, board.black_knights, board.black_bishops, board.black_queens, board.black_king];


    let mut bitboard_to_piecetype:HashMap<u64, String> = HashMap::<u64, String>::new(); // https://doc.rust-lang.org/book/ch08-03-hash-maps.html

    /*Represent pieces as colorPIECETYPE, w for white / b for black, and uppercase first letter of piece name
    (exception: knight, which gets uppercase N, since K is used by king.) */

    bitboard_to_piecetype.insert(u64::from(board.white_pawns), String::from("wP"));
    bitboard_to_piecetype.insert(u64::from(board.white_rooks), String::from("wR"));
    bitboard_to_piecetype.insert(u64::from(board.white_knights), String::from("wN"));
    bitboard_to_piecetype.insert(u64::from(board.white_bishops), String::from("wB"));
    bitboard_to_piecetype.insert(u64::from(board.white_queens), String::from("wQ"));
    bitboard_to_piecetype.insert(u64::from(board.white_king), String::from("wK"));
    bitboard_to_piecetype.insert(u64::from(board.black_pawns), String::from("bP"));
    bitboard_to_piecetype.insert(u64::from(board.black_rooks), String::from("bR"));
    bitboard_to_piecetype.insert(u64::from(board.black_knights), String::from("bN"));
    bitboard_to_piecetype.insert(u64::from(board.black_bishops), String::from("bB"));
    bitboard_to_piecetype.insert(u64::from(board.black_queens), String::from("bQ"));
    bitboard_to_piecetype.insert(u64::from(board.black_king), String::from("bK"));


    let mut entire_board: [String; 64] = std::array::from_fn(|_| ".".to_string());

    // fill the board

    for i in 0..12 {

        for j in 0..64 {

            if (boards[i] >> j & 1) != 0 { // checks if boards[i] is occupied
 
                entire_board[j] = bitboard_to_piecetype.get(&boards[i]).unwrap().to_owned(); // &Option<String> to String https://stackoverflow.com/questions/72840654/get-string-from-optionstring-in-rust
            }
        }
    }


    // print the board
    /* The first square to be printed should be a8, so we get
    a8 b8 ... h8
    a7 b7 ... h7*/ 


    for i in (0..8).rev() { // print top to down

        for j in 0..8 { // print left to right
            print!("{} ",entire_board[i*8+j]);
        }
        print!("{}", "\n\n");
    

    }
    print!("{}", "\n\n");

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