use osen_chess::piece::File;
use osen_chess::piece::Rank;

use osen_chess::ChessError;
use osen_chess::Game;
use osen_chess::GameState;

#[test]
fn pawn_make_move() {
    let mut game = Game::default();

    let m = game.make_move((Rank::A, File::Two), (Rank::A, File::Four));
    println!("{}", game);
    assert_eq!(Ok(()), m);
    assert!(matches!(game.get_state(), GameState::InProgress));

    let m = game.make_move((Rank::A, File::Seven), (Rank::A, File::Six));
    println!("{}", game);
    assert_eq!(Ok(()), m);
    assert!(matches!(game.get_state(), GameState::InProgress));

    let m = game.make_move((Rank::A, File::Six), (Rank::A, File::Five));
    println!("{}", game);
    assert_eq!(Err(ChessError::MismatchedColor), m);
    assert!(matches!(game.get_state(), GameState::InProgress));
}

#[test]
fn pawn_possible_moves() {
    let game = Game::default();
    let moves = game.get_possible_moves((Rank::A, File::Two));
    println!("{moves:?}");

    assert_eq!(moves, Some(vec![(Rank::A, File::Three), (Rank::A, File::Four)]));
}
