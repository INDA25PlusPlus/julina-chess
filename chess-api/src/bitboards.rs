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
