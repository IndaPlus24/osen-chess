mod board;
mod piece;

use crate::{board::Board, piece::{File, Rank}};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameState {
    InProgress,
    Check,
    CheckMate,
    GameOver,
}


pub struct Game {
    state: GameState,
    board: Board,
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl Game {
    /// Initialises a new board with pieces.
    pub fn new() -> Game {
        Game {
            /* initialise board, set active colour to white, ... */
            state: GameState::InProgress,
            board: Board::default(),
        }
    }

    /// If the current game state is InProgress and the move is legal,
    /// move a piece and return the resulting state of the game.
    pub fn make_move(&mut self, from: (File, Rank), to: (File, Rank)) -> Option<GameState> {
        todo!();
    }

    /// Set the piece type that a peasant becames following a promotion.
    pub fn set_promotion(&mut self, piece: (File, Rank)) -> () {
        todo!();
    }

    /// If a piece is standing on the given tile, return all possible
    /// new positions of that piece. Don't forget to the rules for check.
    pub fn get_possible_moves(&self, postion: (File, Rank)) -> Option<Vec<(File, Rank)>> {
        todo!();
    }
}

/// Implement print routine for Game.
///
/// Output example:
/// |:----------------------:|
/// | R  Kn B  K  Q  B  Kn R |
/// | P  P  P  P  P  P  P  P |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | P  P  P  P  P  P  P  P |
/// | R  Kn B  K  Q  B  Kn R |
/// |:----------------------:|

#[cfg(test)]
mod tests {
    use crate::piece::File;
    use crate::piece::Piece;
    use crate::piece::PieceColor;
    use crate::piece::Rank;

    use super::Game;
    use super::GameState;

    // check that game state is in progress after initialisation
    #[test]
    fn game_in_progress_after_init() {
        let game = Game::default();

        println!("{:?}", game.board);

        assert_eq!(game.state, GameState::InProgress);
    }

    #[test]
    fn get_position_of_piece() {
        let game = Game::default();

        let p =game.board.get_piece_at((File::One, Rank::A));
        assert_eq!(p, PieceColor::White(Piece::Rook))
    }
}
