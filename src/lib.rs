mod board;
mod piece;

use crate::{
    board::Board,
    piece::{File, Rank},
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameState {
    InProgress,
    Check,
    CheckMate,
    GameOver,
}

#[derive(Debug)]
pub enum GameTurn {
    White,
    Black,
}

#[derive(Debug)]
pub enum ChessError {
    InvaildMove,
}

pub struct Game {
    state: GameState,
    turn: GameTurn,
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
            turn: GameTurn::White,
            board: Board::default(),
        }
    }

    /// If the current game state is InProgress and the move is legal,
    /// move a piece and return the resulting state of the game.
    pub fn make_move(&mut self, from: (File, Rank), to: (File, Rank)) -> Result<GameState, ChessError> {
        let piece_color = self.board.get_piece_at(&from);
        let piece = match self.turn {
            GameTurn::White => match piece_color {
                piece::PieceColor::White(p) => p,
                piece::PieceColor::Black(_) | piece::PieceColor::Empty => return Err(ChessError::InvaildMove),
            },
            GameTurn::Black => match piece_color {
                piece::PieceColor::Black(p) => p,
                piece::PieceColor::White(_) | piece::PieceColor::Empty => return Err(ChessError::InvaildMove),
            },
        };

        let moves = piece.get_move_set(&from);
        let (f, r) = to;
        if moves.contains(&(i8::from(&f), i8::from(&r))) {
            self.board.set_piece_at(&(f, r), piece_color);
            
            return Ok(GameState::InProgress); 
        }

        Err(ChessError::InvaildMove)
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

        println!("{}", game.board);

        assert_eq!(game.state, GameState::InProgress);
    }

    #[test]
    fn get_position_of_piece() {
        let game = Game::default();

        let p = game.board.get_piece_at(&(File::One, Rank::A));
        assert_eq!(p, PieceColor::White(Piece::Rook))
    }
}
