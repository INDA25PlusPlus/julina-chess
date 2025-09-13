// multiple source files https://www.youtube.com/watch?v=lx5r7yzl1Ps

pub mod bitboards;
pub mod perform_moves;
pub mod legal_moves;
pub mod state;


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
    use crate::state::GameState;

    #[test]
    fn test_pawn_moves() {

        let mut state = GameState::new();

        let mut result = legal_moves::pawn_moves(1<<8, &state);
        assert_eq!(result, 1<<16 | 1<<24);


        state.white_to_move = false;

        result = legal_moves::pawn_moves(1<<55, &state);
        //dbg_print_board(legal_moves::pawn_moves(1<<55, &state));
        assert_eq!(result, 1<<47 | 1<<39);

        // let result = legal_moves::pawn_moves(1<<60) ;
        // assert_eq!(result, 0);
        //dbg_print_board(legal_moves::pawn_moves(1<<8 | 1<<9, &board, 0));
    }

    #[test]
    fn test_king_moves() {

        let state = GameState::new();

        legal_moves::king_moves(1<<30, &state);
        //dbg_print_board(legal_moves::king_moves(1<<60, &board, 1));
    }

    #[test]
    fn test_knight_moves() {

        let state = GameState::new();

        legal_moves::knight_moves(1<<16, &state);
    }

    #[test]
    fn test_rook_moves() {

        let state = GameState::new();
        legal_moves::rook_moves(1<<16, &state);
        //dbg_print_board(legal_moves::rook_moves(1<<24 | 1<<34));
        //dbg_print_board(legal_moves::helper_rook_moves(53, &state));

    }

    #[test]
    fn test_bishop_moves() {
        let state = GameState::new();
        legal_moves::bishop_moves(1<<0, &state);
        legal_moves::bishop_moves(1<<8, &state);
        legal_moves::bishop_moves(1<<63, &state);
        //dbg_print_board(legal_moves::bishop_moves(1<<53, &state));
        // dbg_print_board(legal_moves::bishop_moves(1<<28));
    }

    #[test]
    fn test_queen_moves() {

        let state = GameState::new();

        legal_moves::queen_moves(1<<16, &state);
        // dbg_print_board(legal_moves::queen_moves(1<<53, &state));

    }

    #[test]
    fn test_print_board(){

        let state = GameState::new(); // start configuration

        bitboards::print_board(&state.board); // print start config
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

        assert_eq!(perform_moves::is_check(&state), false); 
    }



    // #[test]
    // fn test_make_move() {

    //     let mut state = GameState::new();

    //     perform_moves::make_move(12, 28, &mut state); // e2-e4
    
    //     perform_moves::make_move(52, 36, &mut state); // e7-e5

    //     perform_moves::make_move(5, 26, &mut state); //bf1-c4

    //     perform_moves::make_move(51, 43, &mut state); // d7-d6

    //     perform_moves::make_move(26, 53, &mut state); //bc4-f7+


    //     let result = perform_moves::is_legal(55, 47, &state); // doesnt handle the check --> not legal

    //     assert_eq!(result, false);

    //     state.white_to_move = true;
    //     assert_eq!(perform_moves::is_check(&state), true);
    //     state.white_to_move = false;
    //     assert_eq!(perform_moves::is_check(&state), false);

    //     // print board
    //     bitboards::print_board(&state.board);
    // }


    #[test]
    fn test_is_checkmate_stalemate() {

        let mut state = GameState::new();

        perform_moves::make_move(12, 28, &mut state, true); // e2-e4
    
        perform_moves::make_move(52, 36, &mut state, true); // e7-e5

        perform_moves::make_move(5, 26, &mut state, true); //bf1-c4

        perform_moves::make_move(51, 43, &mut state, true); // d7-d6

        perform_moves::make_move(3, 21, &mut state, true); // Qe1-f3

        perform_moves::make_move(57, 42, &mut state, true); // Nb8-c6

        perform_moves::make_move(21, 53, &mut state, true); // Qf3-f7#

        // state.white_to_move = true;
        // dbg_print_board(legal_moves::queen_moves(1<<53, &state));
        // println!("Black occupied: \n");
        // dbg_print_board(state.board.black_occupied);
        // println!("Black occupied: \n");
        // dbg_print_board(state.board.white_occupied);
        

        state.white_to_move = true;
        assert_eq!(perform_moves::is_checkmate_stalemate(&mut state), true);

        // // print board
        // bitboards::print_board(&state.board);
    }

    #[test]
    fn test_en_passant() {

        let mut state = GameState::new();

        let move_made = perform_moves::make_move(12, 28, &mut state, true); // e2-e4
        assert_eq!(move_made, true);
        let move_made = perform_moves::make_move(57, 42, &mut state, true); // Nb8-c6
        assert_eq!(move_made, true);
        let move_made = perform_moves::make_move(28, 36, &mut state, true); // e4-e5
        assert_eq!(move_made, true);
        let move_made = perform_moves::make_move(51, 35, &mut state, true); // d7-d5
        assert_eq!(move_made, true);
        let move_made = perform_moves::make_move(36, 43, &mut state, true); // e5xd6
        assert_eq!(move_made, true);

        bitboards::print_board(&state.board);
    }


    
}
