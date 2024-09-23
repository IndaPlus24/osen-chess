mod board;
mod piece;

use std::ops::Not;

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

#[derive(Debug, Copy, Clone)]
struct KingPos {
    white: (File, Rank),
    black: (File, Rank),
}

impl Default for KingPos {
    fn default() -> Self {
        Self {
            white: (File::One, Rank::E),
            black: (File::Eight, Rank::E),
        }
    }
}

/// Game turn
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum GameTurn {
    White,
    Black,
}

impl Not for GameTurn {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            GameTurn::White => GameTurn::Black,
            GameTurn::Black => GameTurn::White,
        }
    }
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
    king_pos: KingPos,
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
            king_pos: KingPos::default(),
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
                PieceColor::White(p) => p,
                PieceColor::Black(_) => return Err(ChessError::DeSyncedTurnColor),
                PieceColor::Empty => return Err(ChessError::EmptySpace),
            },
            GameTurn::Black => match piece_color {
                PieceColor::Black(p) => p,
                PieceColor::White(_) => return Err(ChessError::DeSyncedTurnColor),
                PieceColor::Empty => return Err(ChessError::EmptySpace),
            },
        };

        let moves = piece.get_possible_moves(&self.board, &self.turn, &from);
        if moves.contains(&to) {
            // Test move piece
            let mut test_board = self.board.clone();
            test_board.set_piece_at(&to, piece_color);
            test_board.set_piece_at(&from, PieceColor::Empty);

            let mut king_pos = self.king_pos;

            // if king moves update king pos
            if piece == Piece::King {
                match self.turn {
                    GameTurn::White => king_pos.white = to,
                    GameTurn::Black => king_pos.black = to,
                }
            }

            let king_pos = match self.turn {
                GameTurn::White => king_pos.white,
                GameTurn::Black => king_pos.black,
            };

            // Test for check
            if self.is_check(&test_board, &self.turn, &king_pos) {
                return Err(ChessError::InvalidMove);
            }

            // Move piece
            self.board.set_piece_at(&to, piece_color);
            self.board.set_piece_at(&from, PieceColor::Empty);

            // Check for promotion
            let state = self.board.check_promotion(&to, &self.turn);
            // if promotion; return early to promote
            if let GameState::Promotion(pos) = state {
                return Ok(GameState::Promotion(pos));
            }

            // Check if in check
            let king_pos = match self.turn {
                GameTurn::White => self.king_pos.black,
                GameTurn::Black => self.king_pos.white,
            };
            if self.is_check(&test_board, &!self.turn, &king_pos) {
                self.next_turn();
                return Ok(GameState::Check);
            }

            // Switch turn
            self.next_turn();
            return Ok(GameState::InProgress);
        }

        Err(ChessError::InvalidMove)
    }

    fn is_check(&self, board: &Board, turn: &GameTurn, king_pos: &(File, Rank)) -> bool {
        Piece::Queen
            .get_possible_moves(&self.board, turn, king_pos)
            .into_iter()
            .map(|pos| (board.get_piece_at(&pos), pos))
            .filter_map(|(p, pos)| match p {
                PieceColor::White(piece) => match turn {
                    GameTurn::White => None,
                    GameTurn::Black => Some((piece, pos)),
                },
                PieceColor::Black(piece) => match turn {
                    GameTurn::White => Some((piece, pos)),
                    GameTurn::Black => None,
                },
                PieceColor::Empty => None,
            })
            .map(|(p, pos)| p.get_possible_moves(&self.board, turn, &pos))
            .any(|m| m.contains(king_pos))
    }

    fn next_turn(&mut self) {
        self.turn = !self.turn
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
                Some(piece.get_possible_moves(&self.board, &self.turn, &postion))
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
