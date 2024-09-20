use std::{fmt::Display, usize};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PieceColor {
    White(Piece),
    Black(Piece),
    Empty,
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

#[derive(Debug)]
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

impl From<File> for i8 {
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

impl From<&File> for i8 {
    fn from(value: &File) -> Self {
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

impl From<File> for usize {
    fn from(val: File) -> Self {
        match val {
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

impl From<&File> for usize {
    fn from(val: &File) -> Self {
        match val {
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

#[derive(Debug)]
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

impl From<Rank> for usize {
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

impl From<&Rank> for usize {
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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Piece {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Piece::Pawn => write!(f, "P"),
            Piece::Rook => write!(f, "R"),
            Piece::Knight => write!(f, "k"),
            Piece::Bishop => write!(f, "B"),
            Piece::Queen => write!(f, "Q"),
            Piece::King => write!(f, "K"),
        }
    }
}

impl Piece {
    pub fn get_move_set(self, pos: &(File, Rank)) -> Vec<(i8, i8)> {
        match self {
            Piece::Pawn => vec![(0, 1), (0, 2), (-1, 1), (1, 1)],
            Piece::Rook => vec![(0, 1), (1, 0), (0, -1), (-1, 0)],
            Piece::Knight => vec![
                (1, 2),
                (2, -1),
                (1, -2),
                (-1, -2),
                (-2, -1),
                (-2, 1),
                (-1, 2),
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

    pub fn get_possible_moves(self, game: &Game, pos: &(File, Rank)) -> Vec<(File, Rank)> {
        let len = match self {
            Piece::Pawn => PieceLen::Two,
            Piece::Knight => PieceLen::One,
            Piece::Bishop | Piece::Rook | Piece::Queen => PieceLen::Infinity,
            Piece::King => PieceLen::One,
        };

        let move_dirs = self.get_move_set();

        let moves: Vec<(File, Rank)> = move_dirs
            .into_iter()
            .map(|dir| add_along_dir(dir, pos, &len))
            .map(|list| {
                list.into_iter()
                    .map_while(|pos| match game.board.get_piece_at(&pos) {
                        PieceColor::White(_) => match game.turn {
                            GameTurn::White => None,
                            GameTurn::Black => Some(pos),
                        },
                        PieceColor::Black(_) => match game.turn {
                            GameTurn::White => Some(pos),
                            GameTurn::Black => None,
                        },
                        PieceColor::Empty => Some(pos),
                    })
                    .collect::<Vec<(File, Rank)>>()
            })
            .collect::<Vec<Vec<(File, Rank)>>>()
            .concat();

        moves
    }
}

fn add_along_dir(dir: (i8, i8), pos: &(File, Rank), len: &PieceLen) -> Vec<(File, Rank)> {
    let len: i8 = match len {
        PieceLen::One => 1,
        PieceLen::Two => 2,
        PieceLen::Infinity => 8,
    };
    let (p_x, p_y) = (i8::from(pos.0), i8::from(pos.1));

    let (x, y) = dir;

    (0..len)
        .map(|i| (x * i, y * i))
        .map_while(|(x, y)| {
            Some((
                p_x.checked_add(x)?.try_into().ok()?,
                p_y.checked_add(y)?.try_into().ok()?,
            ))
        })
        .collect()
}

#[derive(Debug)]
enum PieceLen {
    One,
    Two,
    Infinity,
}
