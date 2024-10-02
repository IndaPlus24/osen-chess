pub mod board;
pub mod piece;

use std::{fmt::Display, ops::Not};

use piece::{Piece, PieceColor};

use crate::{
    board::Board,
    piece::{File, Rank},
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
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
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
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

impl Not for &GameTurn {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            GameTurn::White => &GameTurn::Black,
            GameTurn::Black => &GameTurn::White,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum ChessError {
    /// Moved to a unreachable position
    InvalidMove,
    /// PieceColor mismatch from turn color for Game struct
    MismatchedColor,
    /// Position out of bounds on board
    OutOfBounds,
    /// Selecting a empty space
    EmptySpace,
    /// Funktion called in wrong game state
    InvalidGameState,
    /// Board in check, need to move king
    InCheck,
}

/// Game
#[derive(Debug, Clone)]
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

    /// Returns a piece on the board
    pub fn get_piece_at(&self, pos: &(Rank, File)) -> PieceColor {
        self.board.get_piece_at(&(pos.0.into(), pos.1.into()))
    }

    /// If the current game state is InProgress and the move is legal,
    /// move a piece and sets the state of the game. Or returns the move error
    pub fn make_move(&mut self, from: (Rank, File), to: (Rank, File)) -> Result<(), ChessError> {
        // convert to u8
        let from: (u8, u8) = (from.0.into(), from.1.into());
        let to: (u8, u8) = (to.0.into(), to.1.into());

        match self.state {
            GameState::InProgress => (),
            GameState::Check => {
                if self.board.get_piece_at(&from) != self.king_piece() {
                    return Err(ChessError::InCheck);
                }
            }
            _ => return Err(ChessError::InvalidGameState),
        }

        let piece_color = self.board.get_piece_at(&from);
        let piece = match self.turn {
            GameTurn::White => match piece_color {
                PieceColor::White(p) => p,
                PieceColor::Black(_) => return Err(ChessError::MismatchedColor),
                PieceColor::Empty => return Err(ChessError::EmptySpace),
            },
            GameTurn::Black => match piece_color {
                PieceColor::Black(p) => p,
                PieceColor::White(_) => return Err(ChessError::MismatchedColor),
                PieceColor::Empty => return Err(ChessError::EmptySpace),
            },
        };

        let moves = piece.get_possible_moves(&self.board, &self.turn, &from);
        println!("{moves:?}");

        // if trying to move to non-possible space
        if !moves.contains(&to) {
            return Err(ChessError::InvalidMove);
        }

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
        if self.board.is_check(&self.turn, &king_pos) {
            self.next_turn();
            self.state = GameState::Check;
            return Ok(());
        }

        // Switch turn
        self.next_turn();
        self.state = GameState::InProgress;
        Ok(())
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

    /// Get a refrence to the board, useful to be able to iterator over.
    pub fn get_board(&self) -> &Board {
        &self.board
    }

    fn king_piece(&self) -> PieceColor {
        match self.turn {
            GameTurn::White => self.board.get_piece_at(&self.king_pos.white),
            GameTurn::Black => self.board.get_piece_at(&self.king_pos.black),
        }
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
    pub fn get_possible_moves(&self, position: (Rank, File)) -> Option<Vec<(Rank, File)>> {
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
    use crate::ChessError;
    use crate::Game;
    use crate::GameState;
    use crate::GameTurn;
    use crate::KingPos;

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

        assert_eq!(
            GameState::Promotion((Rank::A, File::Eight)),
            game.get_state()
        );

        if let GameState::Promotion(_) = game.get_state() {
            let _ = game.set_promotion(Piece::Queen);
            assert_eq!(GameState::InProgress, game.get_state())
        }
    }

    #[test]
    fn rook_move_test() {
        let mut board = Board::default();
        board.set_piece_at(&(7, 1), PieceColor::White(Piece::Rook));

        let king_pos = KingPos::default();
        let game = Game::new(GameTurn::White, board, king_pos);
        let moves = game.get_possible_moves((Rank::H, File::Seven));
        println!("{game}");
        println!("{moves:?}");

        assert_eq!(
            moves,
            Some(vec![
                (Rank::H, File::Six),
                (Rank::H, File::Five),
                (Rank::H, File::Four),
                (Rank::H, File::Three),
                (Rank::H, File::Eight),
                (Rank::G, File::Seven),
            ])
        )
    }

    #[test]
    fn check_test() {
        let mut board = Board::new(None);
        board.set_piece_at(&(0, 4), PieceColor::White(Piece::King));
        board.set_piece_at(&(4, 5), PieceColor::Black(Piece::Queen));
        let king_pos = crate::KingPos {
            black: (4, 0),
            white: (0, 4),
        };
        let mut game = Game::new(GameTurn::Black, board, king_pos);
        println!("{}", game);

        let _ = game.make_move((Rank::E, File::Three), (Rank::E, File::Four));

        println!("{}", game);

        assert_eq!(game.state, GameState::Check)
    }

    #[test]
    fn check_knight_test() {
        let mut board = Board::new(None);
        board.set_piece_at(&(0, 4), PieceColor::Black(Piece::King));
        board.set_piece_at(&(3, 3), PieceColor::White(Piece::Knight));
        let king_pos = crate::KingPos {
            white: (4, 0),
            black: (0, 4),
        };
        let mut game = Game::new(GameTurn::White, board, king_pos);
        println!("{}", game);

        let _ = game.make_move((Rank::D, File::Five), (Rank::C, File::Three));

        println!("{}", game);

        assert_eq!(game.state, GameState::Check)
    }
}
