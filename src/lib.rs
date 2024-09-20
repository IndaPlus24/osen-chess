mod board;
mod piece;

use piece::PieceColor;

use crate::{
    board::Board,
    piece::{File, Rank},
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameState {
    InProgress,
    Promotion,
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
    InvalidMove,
    DeSyncedTurnColor,
    OutOfBounds,
    EmptySpace,
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
    pub fn make_move(
        &mut self,
        from: (File, Rank),
        to: (File, Rank),
    ) -> Result<GameState, ChessError> {
        let piece_color = self.board.get_piece_at(&from);
        let piece = match self.turn {
            GameTurn::White => match piece_color {
                piece::PieceColor::White(p) => p,
                piece::PieceColor::Black(_) => return Err(ChessError::DeSyncedTurnColor),
                piece::PieceColor::Empty => return Err(ChessError::EmptySpace),
            },
            GameTurn::Black => match piece_color {
                piece::PieceColor::Black(p) => p,
                piece::PieceColor::White(_) => {
                    return Err(ChessError::DeSyncedTurnColor)
                },
                piece::PieceColor::Empty => return Err(ChessError::EmptySpace),
            },
        };

        let moves = piece.get_possible_moves(self, &from);
        if moves.contains(&to) {
            self.board.set_piece_at(&to, piece_color);
            self.board.set_piece_at(&from, PieceColor::Empty);

            // Check if in check
            // Check for promotion

            self.next_turn();
            return Ok(GameState::InProgress);
        }

        Err(ChessError::InvalidMove)
    }

    fn next_turn(&mut self) {
        match self.turn {
            GameTurn::White => self.turn = GameTurn::Black,
            GameTurn::Black => self.turn = GameTurn::White,
        }
    }

    /// Set the piece type that a peasant becames following a promotion.
    pub fn set_promotion(&mut self, piece: (File, Rank)) {
        todo!();
    }

    /// If a piece is standing on the given tile, return all possible
    /// new positions of that piece. Don't forget to the rules for check.
    pub fn get_possible_moves(&self, postion: (File, Rank)) -> Result<Vec<(File, Rank)>, ChessError> {
        match self.board.get_piece_at(&postion) {
            PieceColor::White(piece) | PieceColor::Black(piece) => {
                Ok(piece.get_possible_moves(self, &postion))
            },
            PieceColor::Empty => Err(ChessError::EmptySpace),
        }
    }
}

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
