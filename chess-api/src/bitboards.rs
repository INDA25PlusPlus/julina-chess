/*
Represent the board and pieces using bitboards, one for each piece and color combination,
as suggested in https://en.wikipedia.org/wiki/Bitboard.

That is, there's one bitboard for white pawns (0 means no white pawn, 1 means white pawn), one
bitboard for black knights, etc. (With a single bitboard you would only know whether a piece occupies
the square, but no information of the type and color of the piece).

Bit 0 represents square a1, bit 2 represents c1 and so on, (from left to right, bottom to top).

const u64 example = 0b0000000000000000000000000000000000000000000000000000000000000000;
In example, the right-most bit (bit 0) represents the square a1. Bit 59 is d8, bit 60 e8 ...

The occupation of the squares is initialized based on the starting position (const) of a chess game.


*/

// configuration of pieces in starting position, used to initialize Board struct

const WHITE_PAWN_INIT: u64 = 0b0000000000000000000000000000000000000000000000001111111100000000;
const WHITE_ROOK_INIT: u64 = 0b0000000000000000000000000000000000000000000000000000000010000001;
const WHITE_KNIGHT_INIT: u64 = 0b0000000000000000000000000000000000000000000000000000000001000010;
const WHITE_BISHOP_INIT: u64 = 0b0000000000000000000000000000000000000000000000000000000000100100;
const WHITE_QUEEN_INIT: u64 = 0b0000000000000000000000000000000000000000000000000000000000001000;
const WHITE_KING_INIT: u64 = 0b0000000000000000000000000000000000000000000000000000000000010000;

const BLACK_PAWN_INIT: u64 = 0b0000000011111111000000000000000000000000000000000000000000000000;
const BLACK_ROOK_INIT: u64 = 0b1000000100000000000000000000000000000000000000000000000000000000;
const BLACK_KNIGHT_INIT: u64 = 0b0100001000000000000000000000000000000000000000000000000000000000;
const BLACK_BISHOP_INIT: u64 = 0b0010010000000000000000000000000000000000000000000000000000000000;
const BLACK_QUEEN_INIT: u64 = 0b0000100000000000000000000000000000000000000000000000000000000000;
const BLACK_KING_INIT: u64 = 0b0001000000000000000000000000000000000000000000000000000000000000;

const WHITE_OCCUPIED: u64 = 0b0000000000000000000000000000000000000000000000001111111111111111;
const BLACK_OCCUPIED: u64 = 0b1111111111111111000000000000000000000000000000000000000000000000;


#[derive(Clone)]
pub struct Board {

    pub white_pawns: u64,
    pub white_rooks: u64,
    pub white_knights: u64,
    pub white_bishops: u64,
    pub white_queens: u64,
    pub white_king: u64,

    pub black_pawns: u64,
    pub black_rooks: u64,
    pub black_knights: u64,
    pub black_bishops: u64,
    pub black_queens: u64,
    pub black_king: u64,

    pub white_occupied: u64,
    pub black_occupied: u64,
}


impl Board { // https://doc.rust-lang.org/stable/book/ch05-03-method-syntax.html, https://stdrs.dev/nightly/x86_64-unknown-linux-gnu/std/keyword.self.html
    pub fn new() -> Self {
        Self {
            white_pawns: WHITE_PAWN_INIT,
            white_rooks: WHITE_ROOK_INIT,
            white_knights: WHITE_KNIGHT_INIT,
            white_bishops: WHITE_BISHOP_INIT,
            white_queens: WHITE_QUEEN_INIT,
            white_king: WHITE_KING_INIT,

            black_pawns: BLACK_PAWN_INIT,
            black_rooks: BLACK_ROOK_INIT,
            black_knights: BLACK_KNIGHT_INIT,
            black_bishops: BLACK_BISHOP_INIT,
            black_queens: BLACK_QUEEN_INIT,
            black_king: BLACK_KING_INIT,

            white_occupied: WHITE_OCCUPIED,
            black_occupied: BLACK_OCCUPIED,
        }
    }

    pub fn reset(&mut self) {
        *self = Board::new();
    }
}




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

