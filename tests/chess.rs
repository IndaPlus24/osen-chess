use osen_chess::piece::File;
use osen_chess::piece::Rank;

use osen_chess::Game;
use osen_chess::GameState;

#[test]
fn pawn_make_move() {
    let mut game = Game::default();

    let game_state = game.make_move((Rank::A, File::Two), (Rank::A, File::Four));
    println!("{}", game.get_board());
    println!("{game_state:?}");
    assert!(matches!(game_state, Ok(GameState::InProgress)))
}
