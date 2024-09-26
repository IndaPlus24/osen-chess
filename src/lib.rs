pub mod board;
pub mod piece;

use std::{fmt::Display, ops::Not};

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
    Promotion((Rank, File)),
    /// Board in check
    Check,
    /// Game over
    GameOver,
}

/// The current postion of kings, also used in initializeing game board
#[derive(Debug, Copy, Clone)]
pub(crate) struct KingPos {
    black: (u8, u8),
    white: (u8, u8),
}

impl Default for KingPos {
    fn default() -> Self {
        Self {
            black: (0, 4),
            white: (7, 4),
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

#[derive(Debug, PartialEq, Eq)]
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
    /// Set yourself in check
    CheckPos,
}

pub struct Game {
    state: GameState,
    turn: GameTurn,
    board: Board,
    king_pos: KingPos,
}

impl Default for Game {
    fn default() -> Self {
        Game {
            /* initialise board, set active colour to white, ... */
            state: GameState::InProgress,
            turn: GameTurn::White,
            board: Board::default(),
            king_pos: KingPos::default(),
        }
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.board)
    }
}

impl Game {
    // Testing
    pub(crate) fn new(turn: GameTurn, board: Board, king_pos: KingPos) -> Game {
        Game {
            state: GameState::InProgress,
            turn,
            board,
            king_pos,
        }
    }

    /// If the current game state is InProgress and the move is legal,
    /// move a piece and return the resulting state of the game.
    pub fn make_move(
        &mut self,
        from: (Rank, File),
        to: (Rank, File),
    ) -> Result<(), ChessError> {
        // convert to u8
        let from: (u8, u8) = (from.0.into(), from.1.into());
        let to: (u8, u8) = (to.0.into(), to.1.into());

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
        println!("{moves:?}");

        // if trying to move to impossible space
        if !moves.contains(&to) {
            return Err(ChessError::InvalidMove);
        }

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
        // if self.is_check(&test_board, &self.turn, &king_pos) {
        //     return Err(ChessError::CheckPos);
        // }

        // Move piece
        self.board.set_piece_at(&to, piece_color);
        self.board.set_piece_at(&from, PieceColor::Empty);

        // Check for promotion
        let state = self.board.check_promotion(&to, &self.turn);
        // if promotion; return early to promote
        if let Some(GameState::Promotion(pos)) = state {
            self.state = GameState::Promotion(pos);
            return Ok(());
        }

        // Check if in check
        let king_pos = match self.turn {
            GameTurn::White => self.king_pos.black,
            GameTurn::Black => self.king_pos.white,
        };
        // if self.is_check(&self.board, &!self.turn, &king_pos) {
        //     self.next_turn();
        //     return Ok(GameState::Check);
        // }

        // Switch turn
        self.next_turn();
        self.state = GameState::InProgress;
        Ok(())
    }

    fn is_check(&self, board: &Board, turn: &GameTurn, king_pos: &(u8, u8)) -> bool {
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

    pub fn get_state(&self) -> GameState {
        self.state
    }

    /// Set the piece type that a pawn becomes following a promotion.
    pub fn set_promotion(&mut self, piece: Piece) -> Result<(), ChessError> {
        println!("{:?}", self.state);
        let pos = match self.state {
            GameState::Promotion(pos) => pos,
            _ => return Err(ChessError::InvalidGameState),
        };
        let pos = (pos.0.into(), pos.1.into());

        let mut piece_color = self.board.get_piece_at(&pos);
        piece_color.set_piece(piece)?;

        self.board.set_piece_at(&pos, piece_color);

        // Check for check here

        self.next_turn();
        self.state = GameState::InProgress;
        Ok(())
    }

    /// If a piece is standing on the given tile, return all possible
    /// new positions of that piece. Or None if empty space
    pub fn get_possible_moves(&self, position: (File, Rank)) -> Option<Vec<(Rank, File)>> {
        let position = (position.0.into(), position.1.into());
        match self.board.get_piece_at(&position) {
            PieceColor::White(piece) | PieceColor::Black(piece) => {
                let moves = piece.get_possible_moves(&self.board, &self.turn, &position);
                let moves = moves
                    .into_iter()
                    .map(|pos| (pos.0.try_into().unwrap(), pos.1.try_into().unwrap()))
                    .collect();
                Some(moves)
            }
            PieceColor::Empty => None,
        }
    }
}

#[cfg(test)]
mod lib_test {
    use crate::piece::File;
    use crate::piece::Piece;
    use crate::piece::PieceColor;
    use crate::piece::Rank;
    use crate::Board;
    use crate::Game;
    use crate::GameState;
    use crate::GameTurn;

    // check that game state is in progress after initialisation
    #[test]
    fn init_state() {
        let game = Game::default();

        println!("{}", game.board);

        assert_eq!(game.state, GameState::InProgress);
        assert_eq!(game.turn, GameTurn::White);
        assert_eq!(game.board, Board::default());
    }

    #[test]
    fn white_promote() {
        let mut board = Board::new(None);
        board.set_piece_at(&(0, 1), PieceColor::White(Piece::Pawn(false)));
        let king_pos = crate::KingPos {
            black: (4, 0),
            white: (4, 7),
        };
        let mut game = Game::new(GameTurn::White, board, king_pos);
        println!("{}", game.board);

        let _ = game.make_move((Rank::A, File::Seven), (Rank::A, File::Eight));

        println!("{}", game.board);

        assert_eq!(GameState::Promotion((Rank::A, File::Eight)), game.get_state());

        if let GameState::Promotion(_) = game.get_state() {
            let _ = game.set_promotion(Piece::Queen);
            assert_eq!(GameState::InProgress, game.get_state())
        }
    }
}
