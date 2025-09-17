// multiple source files https://www.youtube.com/watch?v=lx5r7yzl1Ps

pub mod bitboards;
pub mod perform_moves;
pub mod legal_moves;
pub mod state;
pub mod visualize;


#[cfg(test)] // kompileras endast när tester körs
mod tests {

    use super::*;
    use crate::state::GameState;
    use crate::state::Color;
    use crate::state::History;

    #[test]
    fn test_pawn_moves() {

        let state = GameState::new();

        let mut result = legal_moves::pawn_moves(1<<8, &state, Color::White);
        assert_eq!(result, 1<<16 | 1<<24);


        result = legal_moves::pawn_moves(1<<55, &state, Color::Black);
        //dbg_print_board(legal_moves::pawn_moves(1<<55, &state));
        assert_eq!(result, 1<<47 | 1<<39);

        // let result = legal_moves::pawn_moves(1<<60) ;
        // assert_eq!(result, 0);
        //dbg_print_board(legal_moves::pawn_moves(1<<8 | 1<<9, &board, 0));
    }

    #[test]
    fn test_king_moves() {

        let state = GameState::new();
        let side = state.side_to_move;

        legal_moves::king_moves(1<<30, &state, side, true);
        //dbg_print_board(legal_moves::king_moves(1<<60, &board, 1));
    }

    #[test]
    fn test_knight_moves() {

        let state = GameState::new();
        let side = state.side_to_move;

        //legal_moves::knight_moves(1<<16, &state);
        let result = legal_moves::knight_moves(1<<0, &state, side); // a1
        assert_eq!(result, 1<<17);
        let result = legal_moves::knight_moves(1<<7, &state, side); // h1
        assert_eq!(result, 1<<22);
    
    }

    #[test]
    fn test_rook_moves() {

        let state = GameState::new();
        let side = state.side_to_move;
        legal_moves::rook_moves(1<<16, &state, side);
        //dbg_print_board(legal_moves::rook_moves(1<<24 | 1<<34));
        //dbg_print_board(legal_moves::helper_rook_moves(53, &state));

    }

    #[test]
    fn test_bishop_moves() {
        let state = GameState::new();
        let side = state.side_to_move;
        legal_moves::bishop_moves(1<<0, &state, side);
        legal_moves::bishop_moves(1<<8, &state, side);
        legal_moves::bishop_moves(1<<63, &state, side);
        //dbg_print_board(legal_moves::bishop_moves(1<<53, &state));
        // dbg_print_board(legal_moves::bishop_moves(1<<28));
    }

    #[test]
    fn test_queen_moves() {

        let state = GameState::new();

        legal_moves::queen_moves(1<<16, &state, state.side_to_move);
        // dbg_print_board(legal_moves::queen_moves(1<<53, &state));

    }

    #[test]
    fn test_print_board(){

        let state = GameState::new(); // start configuration

        visualize::print_board(&state.board); // print start config
    }

    #[test]
    fn test_is_legal() {

        let state = GameState::new(); // start configuration
        
        let mut result = perform_moves::is_legal(12, 28, &state); //legal
        assert_eq!(result, true);
        result = perform_moves::is_legal(50, 40, &state); // not legal
        assert_eq!(result, false);
    }

    #[test]
    fn test_is_check() {

        let state = GameState::new(); // start configuration

        assert_eq!(perform_moves::is_check(&state, state.side_to_move), false); 
    }



    #[test]
    fn test_make_move() {

        let mut state = GameState::new();
        let mut history =  History::new();

        perform_moves::make_move(12, 28, &mut state, &mut history, true); // e2-e4
    
        perform_moves::make_move(52, 36, &mut state, &mut history, true); // e7-e5

        perform_moves::make_move(5, 26, &mut state, &mut history, true); //bf1-c4

        perform_moves::make_move(51, 43, &mut state, &mut history, true); // d7-d6

        perform_moves::make_move(26, 53, &mut state, &mut history, true); //bc4-f7+

        //bitboards::print_board(&state.board);

        assert_eq!(perform_moves::is_check(&state, Color::White), true);
        assert_eq!(perform_moves::is_check(&state, Color::Black), false);


        let result = perform_moves::make_move(55, 47, &mut state, &mut history, true); // doesnt handle the check --> not valid move
        
        assert_eq!(result, false);

    }


    #[test]
    fn test_is_checkmate_stalemate() {

        let mut state = GameState::new();
        let mut history =  History::new();

        perform_moves::make_move(12, 28, &mut state, &mut history,true); // e2-e4
        assert_eq!(state.side_to_move, Color::Black);
    
        perform_moves::make_move(52, 36, &mut state,&mut history,  true); // e7-e5
        assert_eq!(state.side_to_move, Color::White);

        perform_moves::make_move(5, 26, &mut state, &mut history, true); //bf1-c4
        assert_eq!(state.side_to_move, Color::Black);
        
        perform_moves::make_move(51, 43, &mut state, &mut history, true); // d7-d6
        assert_eq!(state.side_to_move, Color::White);

        perform_moves::make_move(3, 21, &mut state, &mut history, true); // Qe1-f3
        assert_eq!(state.side_to_move, Color::Black);

        perform_moves::make_move(57, 42, &mut state, &mut history, true); // Nb8-c6
        assert_eq!(state.side_to_move, Color::White);

        perform_moves::make_move(21, 53, &mut state, &mut history, true); // Qf3-f7#
        assert_eq!(state.side_to_move, Color::Black);

        visualize::print_board(&state.board);

        // state.white_to_move = true;
        // dbg_print_board(legal_moves::queen_moves(1<<53, &state));
        // println!("Black occupied: \n");
        // dbg_print_board(state.board.black_occupied);
        // println!("Black occupied: \n");
        // dbg_print_board(state.board.white_occupied);
        

        state.side_to_move = Color::Black;
        assert_eq!(perform_moves::is_checkmate_stalemate(&mut state), true);

        // // print board
        //bitboards::print_board(&state.board);
    }

    #[test]
    fn test_en_passant() {

        let mut state = GameState::new();
        let mut history =  History::new();

        let move_made = perform_moves::make_move(12, 28, &mut state, &mut history, true); // e2-e4
        assert_eq!(move_made, true);
        let move_made = perform_moves::make_move(57, 42, &mut state, &mut history,true); // Nb8-c6
        assert_eq!(move_made, true);
        let move_made = perform_moves::make_move(28, 36, &mut state,   &mut history, true); // e4-e5
        assert_eq!(move_made, true);
        let move_made = perform_moves::make_move(51, 35, &mut state, &mut history,true); // d7-d5
        assert_eq!(move_made, true);
        let move_made = perform_moves::make_move(36, 43, &mut state, &mut history, true); // e5xd6
        assert_eq!(move_made, true);

    }

    #[test]
    fn test_castling() {

        let mut state = GameState::new();
        let mut history =  History::new();

        // white and black king-side

        let move_made = perform_moves::make_move(12, 28, &mut state, &mut history, true); // e2-e4
        assert_eq!(move_made, true);
        let move_made = perform_moves::make_move(52, 36, &mut state, &mut history, true); // e7-e5
        assert_eq!(move_made, true);
        let move_made = perform_moves::make_move(6, 21, &mut state, &mut history, true); // Ng1-f3
        assert_eq!(move_made, true);
        let move_made = perform_moves::make_move(57, 42, &mut state, &mut history, true); // Nb8-c6
        assert_eq!(move_made, true);
        let move_made =perform_moves::make_move(5, 26, &mut state, &mut history, true); //bf1-c4
        assert_eq!(move_made, true); 
        let move_made =perform_moves::make_move(62, 45, &mut state, &mut history, true); //Ng8-f6
        assert_eq!(move_made, true); 
        let move_made =perform_moves::make_move(4, 6, &mut state, &mut history, true); // 0-0 white
        assert_eq!(move_made, true); 
        let move_made =perform_moves::make_move(61, 52, &mut state, &mut history, true); // Bf8-e7
        assert_eq!(move_made, true); 
        let move_made =perform_moves::make_move(11, 19, &mut state, &mut history, true); // d2-d3
        assert_eq!(move_made, true); 
        let move_made =perform_moves::make_move(60, 62, &mut state, &mut history, true); // 0-0 black
        assert_eq!(move_made, true); 

        //bitboards::print_board(&state.board);

        state.reset();


        // white and black queen-side
        let move_made = perform_moves::make_move(11, 27, &mut state, &mut history, true); // d2-d4
        assert_eq!(move_made, true);
        let move_made = perform_moves::make_move(51, 35, &mut state, &mut history, true); // d7-d5
        assert_eq!(move_made, true);
        let move_made = perform_moves::make_move(2, 29, &mut state, &mut history, true); // Bc1-f4
        assert_eq!(move_made, true);
        let move_made = perform_moves::make_move(57, 42, &mut state, &mut history, true); // Nb8-c6
        assert_eq!(move_made, true);
        let move_made = perform_moves::make_move(1, 18, &mut state, &mut history, true); // Nb1-c3
        assert_eq!(move_made, true);
        let move_made = perform_moves::make_move(58, 44, &mut state, &mut history, true); // Bc8-e6
        assert_eq!(move_made, true);
        let move_made = perform_moves::make_move(3, 11, &mut state, &mut history, true); // Qd1-d2
        assert_eq!(move_made, true);
        let move_made = perform_moves::make_move(59, 51, &mut state, &mut history, true); // Qd8-d7
        assert_eq!(move_made, true);
        let move_made = perform_moves::make_move(4, 2, &mut state, &mut history, true); // 0-0-0 white
        assert_eq!(move_made, true);
        let move_made = perform_moves::make_move(60, 58, &mut state, &mut history, true); // 0-0-0 black
        assert_eq!(move_made, true);

        //bitboards::print_board(&state.board);

    }


    #[test]
    fn test_promotion() {

        let mut state = GameState::new();
        let board = &mut state.board;
        let mut history =  History::new();

        // change position such that we have an endgame of some type
        board.white_bishops = 0;
        board.white_pawns = 0;
        board.white_rooks = 0;
        board.white_knights = 0;
        board.white_occupied = 0;
        board.white_queens = 0;
        board.white_occupied |= 1<<4;

        board.black_pawns &= !(1<<48); // remove pawn from a7
        board.black_pawns |= 1<<8; // add pawn to a2
        board.black_occupied |= 1<<8;
        
        state.side_to_move = Color::Black; // black to move
        
        // test if black can promote pawn on a1.
        let move_made = perform_moves::make_move(8, 0, &mut state, &mut history, true);
        assert_eq!(move_made, true);


        visualize::print_board(&state.board);

    }
    
}
