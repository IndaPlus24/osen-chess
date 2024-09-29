use std::fmt::Display;

use crate::{board::Board, ChessError, GameTurn};

/// A piece color on the board, holing the piece type
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PieceColor {
    White(Piece),
    Black(Piece),
    Empty,
}

impl PieceColor {
    pub(crate) fn set_piece(&mut self, piece: Piece) -> Result<(), ChessError> {
        match self {
            PieceColor::White(p) => *p = piece,
            PieceColor::Black(p) => *p = piece,
            PieceColor::Empty => return Err(ChessError::MismatchedColor),
        };
        Ok(())
    }
}

impl Display for PieceColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PieceColor::White(p) => write!(f, "W{}", p),
            PieceColor::Black(p) => write!(f, "B{}", p),
            PieceColor::Empty => write!(f, "__"),
        }
    }
}

impl From<PieceColor> for GameTurn {
    fn from(value: PieceColor) -> Self {
        match value {
            PieceColor::White(_) => GameTurn::White,
            PieceColor::Black(_) => GameTurn::Black,
            PieceColor::Empty => todo!(),
        }
    }
}

/// The y position on the board
#[derive(Debug, PartialEq, Clone, Copy, Eq, PartialOrd, Ord)]
pub enum File {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

impl TryFrom<i8> for File {
    type Error = ChessError;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(File::One),
            1 => Ok(File::Two),
            2 => Ok(File::Three),
            3 => Ok(File::Four),
            4 => Ok(File::Five),
            5 => Ok(File::Six),
            6 => Ok(File::Seven),
            7 => Ok(File::Eight),
            _ => Err(ChessError::OutOfBounds),
        }
    }
}

impl TryFrom<u8> for File {
    type Error = ChessError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            7 => Ok(File::One),
            6 => Ok(File::Two),
            5 => Ok(File::Three),
            4 => Ok(File::Four),
            3 => Ok(File::Five),
            2 => Ok(File::Six),
            1 => Ok(File::Seven),
            0 => Ok(File::Eight),
            _ => Err(ChessError::OutOfBounds),
        }
    }
}

impl From<File> for u8 {
    fn from(value: File) -> Self {
        match value {
            File::One => 7,
            File::Two => 6,
            File::Three => 5,
            File::Four => 4,
            File::Five => 3,
            File::Six => 2,
            File::Seven => 1,
            File::Eight => 0,
        }
    }
}

/// The x position of the board
#[derive(Debug, PartialEq, Clone, Copy, Eq, PartialOrd, Ord)]
pub enum Rank {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl TryFrom<i8> for Rank {
    type Error = ChessError;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Rank::A),
            1 => Ok(Rank::B),
            2 => Ok(Rank::C),
            3 => Ok(Rank::D),
            4 => Ok(Rank::E),
            5 => Ok(Rank::F),
            6 => Ok(Rank::G),
            7 => Ok(Rank::H),
            _ => Err(ChessError::OutOfBounds),
        }
    }
}

impl TryFrom<u8> for Rank {
    type Error = ChessError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Rank::A),
            1 => Ok(Rank::B),
            2 => Ok(Rank::C),
            3 => Ok(Rank::D),
            4 => Ok(Rank::E),
            5 => Ok(Rank::F),
            6 => Ok(Rank::G),
            7 => Ok(Rank::H),
            _ => Err(ChessError::OutOfBounds),
        }
    }
}

impl From<Rank> for u8 {
    fn from(val: Rank) -> Self {
        match val {
            Rank::A => 0,
            Rank::B => 1,
            Rank::C => 2,
            Rank::D => 3,
            Rank::E => 4,
            Rank::F => 5,
            Rank::G => 6,
            Rank::H => 7,
        }
    }
}

impl From<Rank> for i8 {
    fn from(val: Rank) -> Self {
        match val {
            Rank::A => 0,
            Rank::B => 1,
            Rank::C => 2,
            Rank::D => 3,
            Rank::E => 4,
            Rank::F => 5,
            Rank::G => 6,
            Rank::H => 7,
        }
    }
}

impl From<&Rank> for i8 {
    fn from(val: &Rank) -> Self {
        match val {
            Rank::A => 0,
            Rank::B => 1,
            Rank::C => 2,
            Rank::D => 3,
            Rank::E => 4,
            Rank::F => 5,
            Rank::G => 6,
            Rank::H => 7,
        }
    }
}

/// To indicate if a move is a pawn's first
pub type IsFirstMove = bool;

/// Piece type
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Piece {
    Pawn(IsFirstMove),
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Piece::Pawn(first) => match first {
                true => write!(f, "P"),
                false => write!(f, "p"),
            },
            Piece::Rook => write!(f, "R"),
            Piece::Knight => write!(f, "k"),
            Piece::Bishop => write!(f, "B"),
            Piece::Queen => write!(f, "Q"),
            Piece::King => write!(f, "K"),
        }
    }
}

impl Piece {
    fn get_move_set(&self, turn: &GameTurn) -> Vec<(i8, i8)> {
        let flip_y = match turn {
            GameTurn::White => 1,
            GameTurn::Black => -1,
        };
        match self {
            Piece::Pawn(first_move) => match first_move {
                true => vec![(-1, -flip_y), (1, -flip_y), (0, -flip_y), (0, -2 * flip_y)],
                false => vec![(-1, -flip_y), (1, -flip_y), (0, -flip_y)],
            },
            Piece::Rook => vec![(0, 1), (1, 0), (0, -1), (-1, 0)],
            Piece::Knight => vec![
                (-1, -2),
                (1, -2),
                (2, -1),
                (2, 1),
                (1, 2),
                (-1, 2),
                (-2, 1),
                (-2, -1),
            ],
            Piece::Bishop => vec![(1, 1), (-1, -1), (1, -1), (-1, 1)],
            Piece::King | Piece::Queen => vec![
                (0, 1),
                (1, 1),
                (1, 0),
                (1, -1),
                (0, -1),
                (-1, -1),
                (-1, 0),
                (-1, 1),
            ],
        }
    }

    pub(crate) fn get_possible_moves(
        self,
        board: &Board,
        turn: &GameTurn,
        pos: &(u8, u8),
    ) -> Vec<(u8, u8)> {
        let len = match self {
            Piece::Pawn(_) => PieceLen::One,
            Piece::Knight => PieceLen::One,
            Piece::Bishop | Piece::Rook | Piece::Queen => PieceLen::Infinity,
            Piece::King => PieceLen::One,
        };

        let move_dirs = self.get_move_set(turn);

        if let Piece::Pawn(_) = self {
            let mut moves = vec![];
            let mut move_dirs = move_dirs.into_iter();

            let dir = move_dirs.next().unwrap();
            if let Some(p) = check_pawn_cap_move(pos, &len, &dir, board, turn) {
                moves.push(p);
            }
            let dir = move_dirs.next().unwrap();
            if let Some(p) = check_pawn_cap_move(pos, &len, &dir, board, turn) {
                moves.push(p);
            }

            moves.append(&mut self.collect_along_dirs(board, turn, move_dirs, pos, &len));

            return moves;
        }

        self.collect_along_dirs(board, turn, move_dirs.into_iter(), pos, &len)
    }

    pub(crate) fn collect_along_dirs(
        &self,
        board: &Board,
        turn: &GameTurn,
        move_dirs: std::vec::IntoIter<(i8, i8)>,
        pos: &(u8, u8),
        len: &PieceLen,
    ) -> Vec<(u8, u8)> {
        let moves = move_dirs.map(|dir| add_along_dir(&dir, pos, len));
        println!("{moves:?}");

        moves
            .map(|list| {
                let mut stop = false;
                let res = list
                    .into_iter()
                    .map_while(|dir_pos| {
                        let moves = self.match_along_dir(board, turn, dir_pos);
                        println!("{:?}", moves);
                        if stop {
                            return None;
                        }
                        stop = moves.1;
                        moves.0
                    })
                    .collect::<Vec<(u8, u8)>>();
                println!("{res:?}");
                res
            })
            .collect::<Vec<Vec<(u8, u8)>>>()
            .concat()
    }

    pub(crate) fn match_along_dir(
        &self,
        board: &Board,
        turn: &GameTurn,
        dir_pos: (u8, u8),
    ) -> (Option<(u8, u8)>, bool) {
        match board.get_piece_at(&dir_pos) {
            PieceColor::White(_) => match turn {
                GameTurn::White => (None, true),
                GameTurn::Black => (Some(dir_pos), true),
            },
            PieceColor::Black(_) => match turn {
                GameTurn::White => (Some(dir_pos), true),
                GameTurn::Black => (None, true),
            },
            PieceColor::Empty => (Some(dir_pos), false),
        }
    }
}

pub(crate) fn check_pawn_cap_move(
    pos: &(u8, u8),
    len: &PieceLen,
    dir: &(i8, i8),
    board: &Board,
    turn: &GameTurn,
) -> Option<(u8, u8)> {
    let dir = add_along_dir(dir, pos, len);
    let cap_pos = dir.first()?;
    let board_pos = board.get_piece_at(cap_pos);
    match board_pos {
        PieceColor::White(_) => match turn {
            GameTurn::White => None,
            GameTurn::Black => Some(*cap_pos),
        },
        PieceColor::Black(_) => match turn {
            GameTurn::White => Some(*cap_pos),
            GameTurn::Black => None,
        },
        PieceColor::Empty => None,
    }
}

fn add_along_dir(dir: &(i8, i8), pos: &(u8, u8), len: &PieceLen) -> Vec<(u8, u8)> {
    let len: i8 = match len {
        PieceLen::One => 1,
        // PieceLen::Two => 2,
        PieceLen::Infinity => 8,
    };
    let (p_x, p_y) = pos;
    // println!("pos: {:?}", (p_x, p_y));

    let (x, y) = dir;
    println!("dir: {:?}", (x, y));

    (1..=len)
        .map(|i| (x * i, y * i))
        .map_while(|(x, y)| {
            let r: i8 = *p_x as i8 + x;
            let f: i8 = *p_y as i8 + y;

            if r >= 8 || f >= 8 {
                return None;
            }

            println!("{:?}", (r, f));
            Some((r.try_into().ok()?, f.try_into().ok()?))
        })
        .collect()
}

#[derive(Debug)]
pub(crate) enum PieceLen {
    One,
    // Two,
    Infinity,
}

#[cfg(test)]
mod piece_test {
    use crate::board::Board;
    use crate::piece::PieceLen;
    use crate::Game;
    use crate::GameTurn;

    use super::add_along_dir;
    use super::Piece;
    use super::PieceColor;

    #[test]
    fn add_along_dir_test_down() {
        let dir = add_along_dir(&(0, 1), &(0, 2), &PieceLen::Infinity);
        println!("{dir:?}");
        assert_eq!(dir, vec![(0, 3), (0, 4), (0, 5), (0, 6), (0, 7),])
    }

    #[test]
    fn add_along_dir_test_up() {
        let dir = add_along_dir(&(0, -1), &(0, 3), &PieceLen::Infinity);
        println!("{dir:?}");
        assert_eq!(dir, vec![(0, 2), (0, 1), (0, 0)])
    }

    #[test]
    fn add_along_dir_test_left() {
        let dir = add_along_dir(&(-1, 0), &(3, 0), &PieceLen::Infinity);
        println!("{dir:?}");
        assert_eq!(dir, vec![(2, 0), (1, 0), (0, 0)])
    }

    #[test]
    fn add_along_dir_test_right() {
        let dir = add_along_dir(&(1, 0), &(3, 0), &PieceLen::Infinity);
        println!("{dir:?}");
        assert_eq!(dir, vec![(4, 0), (5, 0), (6, 0), (7, 0)])
    }

    #[test]
    fn match_along_dir_test() {
        let mut board = Board::new(None);
        board.set_piece_at(&(6, 7), PieceColor::White(Piece::Knight));
        board.set_piece_at(&(5, 5), PieceColor::Black(Piece::Queen));
        let dir_pos = (5, 5);
        let piece = board.get_piece_at(&(6, 7));
        if let PieceColor::White(p) = piece {
            let res = p.match_along_dir(&board, &GameTurn::White, dir_pos);
            println!("{res:?}");
            assert_eq!(res, (Some(dir_pos), true))
        };
    }

    #[test]
    fn collect_along_dirs_test_knight() {
        let game = Game::default();
        let pos = (6, 7);
        let piece = game.board.get_piece_at(&pos);
        if let PieceColor::White(piece) = piece {
            println!("{piece:?} at {pos:?}");
            let moves = piece.collect_along_dirs(
                &game.board,
                &GameTurn::White,
                piece.get_move_set(&game.get_turn()).into_iter(),
                &pos,
                &PieceLen::One,
            );
            println!("{moves:?}");
            assert_eq!(moves, vec![(5, 5), (7, 5)])
        }
    }

    #[test]
    fn collect_along_dirs_test_pawn_first_move() {
        let game = Game::default();
        let pos = (6, 6);
        let piece = game.board.get_piece_at(&pos);
        if let PieceColor::White(piece) = piece {
            println!("{piece:?} at {pos:?}");
            let moves = piece.collect_along_dirs(
                &game.board,
                &GameTurn::White,
                piece.get_move_set(&game.get_turn()).into_iter(),
                &pos,
                &PieceLen::One,
            );
            println!("{moves:?}");
            assert_eq!(moves, vec![(5, 5), (7, 5), (6, 5), (6, 4)])
        }
    }

    #[test]
    fn collect_along_dirs_test_pawn() {
        let mut game = Game::default();
        game.board.set_piece_at(&(6, 6), PieceColor::Empty);
        let pos = (6, 5);
        game.board
            .set_piece_at(&pos, PieceColor::White(Piece::Pawn(false)));
        let piece = game.board.get_piece_at(&pos);
        if let PieceColor::White(piece) = piece {
            println!("{piece:?} at {pos:?}");
            let moves = piece.collect_along_dirs(
                &game.board,
                &GameTurn::White,
                piece.get_move_set(&game.get_turn()).into_iter(),
                &pos,
                &PieceLen::One,
            );
            println!("{moves:?}");
            assert_eq!(moves, vec![(5, 4), (7, 4), (6, 4)])
        }
    }
}
