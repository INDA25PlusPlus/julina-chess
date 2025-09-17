/*
GameState represents the full state of the chess game, including storing:
- Board (piece positions)
- Castlig rights
- (if available) en passant square (bitboard with one bit set as the square the opponent's pawn just jumped over)
- whose turn

Allows you to manage the game state cleanly without relying on eg. static mut (threading bugs etc.)
*/

use crate::bitboards::Board;
use std::array;

const MAX_GAME_MOVES: usize = 200;


#[derive(Clone, Copy, PartialEq, Debug)]
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


// Game history taken from: https://rustic-chess.org/board_representation/game_history.html


pub struct History {
    list: [GameState; MAX_GAME_MOVES],
    count: usize,
}

impl History {
    // Create a new history array containing game states.
    pub fn new() -> Self {
        Self {
            list: array::from_fn(|_| GameState::new()),
            count: 0,
        }
    }

    // Put a new game state into the array.
    pub fn push(&mut self, g: GameState) {
        self.list[self.count] = g;
        self.count += 1;
    }

    // Return the last game state and decrement the counter. The game state is
    // not deleted from the array. If necessary, another game state will just
    // overwrite it.
    pub fn pop(&mut self) -> Option<GameState> {
        if self.count > 0 {
            self.count -= 1;
            Some(self.list[self.count].clone()) // must clone since GameState isn't Copy
        } else {
            None
        }
    }

    pub fn get_ref(&self, index: usize) -> &GameState {
        &self.list[index]
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn clear(&mut self) {
        self.count = 0;
    }
}