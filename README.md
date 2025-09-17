# Chess Library (written in Rust) for IndaPlusPlus

## A fully functional chess enginge library in Rust supporting standard chess rules. 


Apart from supporting standard chess rules, including castling, en passant, pawn promotion, and check/checkmate detection, this chess library uses several optimizations for high performance. This includes representing the board and pieces with bitboards, computing legal moves using bitmasks and bitshifting, and having an undo_move() to avoid redundant cloning. 


## Features

* **Bitboard representation** for efficient board state and move calculations.
* **Piece-specific move generation:** pawn, knight, bishop, rook, queen, king.
* **Game state management** with GameState and Board structs.
* **History tracking** for undoing moves and detecting self-check.
* **Legal move validation** including self-check detection.
* **Special moves:**
    * Castling
    * En passant
    * Pawn promotion with interaction for promotion choices
* **Check detection**.
* **Checkmate and stalemate detection**.
* **Little-Endian Rank-File Mapping (LERF mapping)** of chessboard.


## Overview??

## Guide: How to use the library

1. Create a new Rust project with 'cargo new my-project'
2. Edit my-project/Cargo.toml 
    * Add chess-api as a dependency
3. Use the library in my-project/src/main.rs
    * use chess-api

Specifically, you'll need to use:
* chess-api::state::GameState
* chess-api::state::History

Now, you can create a new chess game instance, eg. let state = GameState::new() and
let history = History::new().
* Try for examplel visualizing the board in the terminal using visualize::print_board(&state.board)

**Make a move** by calling the function perform_moves::make_move() with parameters:
* i8 current_square: Integer in range [0, 63] representing the square that the piece you want to move occupies.
    * 0 = a1, 1 = a2, 2 = a3, etc. (LERF mapping)
* i8 target_square: Integer in range [0, 63] representing the square that you want to move your piece to.
* mutable reference of GameState 'state'
* mutable reference of History 'history'
* bool stop_reset: Set to true if you don't want to automatically reset the game if checkmate/stalemate occurs.


## Core Structures

### Board
Tracks the positions of all pieces for both sides using u64 bitboards.
A 1 represents an occupied square, a 0 represents an unoccupied square.

**Fields:**
* white_occupied, black_occupied: Bitboards for all pieces of each color.
* white_pawns, white_knights, white_bishops, white_rooks, white_queens, white_king
* black_pawns, black_knights, black_bishops, black_rooks, black_queens, black_king

#### Bitboard Notes
* Each bit represents a square on the board (0=a1, 63=h8). This chess engine uses LERF-mappig (Little-Endian Rank-File Mapping)
* FILE_A, FILE_H, etc. are masks for board edges, used to prevent wraparound in moves.


### GameState
Inherits from Board and tracks game-specific state.

**Fields:**

* board: Board
* side_to_move: Color — White or Black
* en_passant_mask: u64 — Tracks the en passant square
* Castling rights bools:
    * white_can_castle_kingside, white_can_castle_queenside
    * black_can_castle_kingside, black_can_castle_queenside

#### Color Enum
Represents the player side.

```rust
enum Color {
    White,
    Black
}
```

### History
A stack of previous GameState instances for undo functionality. Credit to: https://rustic-chess.org/front_matter/title.html.


## Functions



### Move validation

* read_cur_square(square: i8, state: &GameState) -> bool
Checks if the user pressed a square occupied by a piece of the same color as state.side_to_move Returns `true` if the square contains a piece belonging to `state.side_to_move`. This allows the digital board to enforce separate checks for the selected piece, similar to the behavior on Lichess or Chess.com, where only pieces of the player whose turn it is can be picked for movement.

* is_legal(cur_square: i8, target_square: i8, state: &GameState) -> bool
Checks if a piece can legally move from cur_square to target_square according to piece movement rules, ignoring self-check.


### Move execution
* make_move(cur_square: i8, target_square: i8, state: &mut GameState, history: &mut History, stop_reset: bool) -> bool
Executes a move if legal, updates history, handles captures, promotions, en passant, and castling, checks for self-check, and updates turn.

* capture(target_mask: u64, board: &mut Board) -> Option<i8>
Removes a piece from the board if present at target_mask. Returns the type of captured piece.

* fill_square(cur_mask: u64, target_mask: u64, board: &mut Board)
Adds the piece present at cur_mask to target_mask on the board.

* empty_square(cur_mask: u64, board: &mut Board)
Removes the piece from the specified square.

* promotion(target_square: i8, state: &mut GameState)
Handles pawn promotion, with interactive console input for the promoted piece.

* en_passant(target_mask: u64, state: &mut GameState)
Performs en passant capture if applicable.

* update_en_passant_square(cur_square: i8, target_square: i8, state: &mut GameState)
Updates en passant state after a pawn moves two squares.

* castle(cur_square: i8, target_square: i8, state: &mut GameState)
Handles rook movement during castling.

* update_castling_rights(cur_square: i8, state: &mut GameState)
Updates castling rights after a king or rook moves.

### Check & Checkmate Detection

* checked_squares(state: &GameState, side_checking: Color) -> u64
Returns all squares currently attacked by side_checking.

* is_check(state: &GameState, side_checking: Color) -> bool
Determines if side_checking is currently giving check to the opponent.

* is_checkmate_stalemate(state: &mut GameState) -> bool
Checks if the current player has no legal moves, signaling checkmate or stalemate.

### Undo Moves
* undo_move(original_square: i8, new_square: i8, state: &mut GameState, side: Color, piece_captured: Option<i8>)
Reverts a move including captured pieces and castling.

* restore_captured_piece(new_mask: u64, state: &mut GameState, side: Color, piece_captured: Option<i8>)
Restores captured pieces to the board.

* undo_castle(original_square: i8, new_square: i8, state: &mut GameState)
Reverts castling rook movement.

### Piece Move Generation
Each function returns a bitboard of all legal target squares for the piece:
* pawn_moves(pos: u64, state: &GameState, side: Color) -> u64
* knight_moves(pos: u64, state: &GameState, side: Color) -> u64
* bishop_moves(pos: u64, state: &GameState, side: Color) -> u64
* bishop_moves(pos: u64, state: &GameState, side: Color) -> u64
* queen_moves(pos: u64, state: &GameState, side: Color) -> u64
* king_moves(pos: u64, state: &GameState, side: Color, include_castling: bool) -> u64

#### Castling helper
* castling(cur_square: i8, state: &GameState) -> u64
Returns squares the king can move to via castling if allowed.


## References

- [Chess Programming Wiki](https://www.chessprogramming.org/)

- [The Rust Programming Language](https://doc.rust-lang.org/book/)

- ChatGPT help with documentation and questions

- Lichess analysis board for creating example positions