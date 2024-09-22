mod board;
mod piece;

use piece::{Piece, PieceColor};

use crate::{
    board::Board,
    piece::{File, Rank},
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameState {
    /// Game in progress
    InProgress,
    /// Waits for promotion of piece
    Promotion((File, Rank)),
    /// Board in check
    Check,
    /// Game over
    GameOver,
}

/// Game turn
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum GameTurn {
    White,
    Black,
}

#[derive(Debug)]
pub enum ChessError {
    /// Moved to a unreachable position
    InvalidMove,
    /// Turn desynced from Game struct
    DeSyncedTurnColor,
    /// Position out of bounds on board
    OutOfBounds,
    /// Selecting a empty space
    EmptySpace,
    /// Funktion called in wrong game state
    InvalidGameState,
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
                piece::PieceColor::White(_) => return Err(ChessError::DeSyncedTurnColor),
                piece::PieceColor::Empty => return Err(ChessError::EmptySpace),
            },
        };

        let moves = piece.get_possible_moves(self, &from);
        if moves.contains(&to) {
            // Move piece
            self.board.set_piece_at(&to, piece_color);
            self.board.set_piece_at(&from, PieceColor::Empty);

            // Check for promotion
            let mut state = self.board.check_promotion(&to, &self.turn);

            // Check if in check
            state = match moves
                .iter()
                .map(|pos| self.board.get_piece_at(pos))
                .filter_map(|p| match p {
                    PieceColor::White(piece) => match self.turn {
                        GameTurn::White => None,
                        GameTurn::Black => match piece {
                            piece::Piece::King => Some(piece),
                            _ => None,
                        },
                    },
                    PieceColor::Black(piece) => match self.turn {
                        GameTurn::White => match piece {
                            piece::Piece::King => Some(piece),
                            _ => None,
                        },
                        GameTurn::Black => None,
                    },
                    PieceColor::Empty => None,
                })
                .next()
                .is_some()
            {
                true => GameState::Check,
                false => state,
            };

            // Switch turn
            self.next_turn();
            return Ok(state);
        }

        Err(ChessError::InvalidMove)
    }

    fn next_turn(&mut self) {
        match self.turn {
            GameTurn::White => self.turn = GameTurn::Black,
            GameTurn::Black => self.turn = GameTurn::White,
        }
    }

    pub fn get_turn(&self) -> GameTurn {
        self.turn
    }

    /// Set the piece type that a pawn becames following a promotion.
    pub fn set_promotion(&mut self, piece: Piece) -> Result<(), ChessError> {
        let pos = match self.state {
            GameState::Promotion(pos) => pos,
            _ => return Err(ChessError::InvalidGameState),
        };

        let mut piece_color = self.board.get_piece_at(&pos);
        piece_color.set_piece(piece)?;

        self.board.set_piece_at(&pos, piece_color);

        Ok(())
    }

    /// If a piece is standing on the given tile, return all possible
    /// new positions of that piece. Or None if empty space
    pub fn get_possible_moves(&self, postion: (File, Rank)) -> Option<Vec<(File, Rank)>> {
        match self.board.get_piece_at(&postion) {
            PieceColor::White(piece) | PieceColor::Black(piece) => {
                Some(piece.get_possible_moves(self, &postion))
            }
            PieceColor::Empty => None,
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
