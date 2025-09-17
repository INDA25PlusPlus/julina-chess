/*
GameState represents the full state of the chess game, including storing:
- Board (piece positions)
- Castlig rights
- (if available) en passant square (bitboard with one bit set as the square the opponent's pawn just jumped over)
- whose turn

Allows you to manage the game state cleanly without relying on eg. static mut (threading bugs etc.)
*/

use crate::bitboards::Board;


#[derive(Clone, Copy, PartialEq)]
pub enum Color {
    White,
    Black,
}

impl Color {

    pub fn opposite(self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}



#[derive(Clone)]
pub struct GameState {
    pub board: Board,
    pub white_can_castle_kingside: bool,
    pub white_can_castle_queenside: bool,
    pub black_can_castle_kingside: bool,
    pub black_can_castle_queenside: bool,
    pub en_passant_mask: u64, 
    pub side_to_move: Color,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            white_can_castle_kingside: true,
            white_can_castle_queenside: true,
            black_can_castle_kingside: true,
            black_can_castle_queenside: true,
            en_passant_mask: 0,
            side_to_move: Color::White,
        }
    }

    pub fn reset(&mut self) {
        *self = GameState::new();
    }
}