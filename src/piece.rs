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
        // let len = match self {
        //     Piece::Pawn => PieceLen::Two,
        //     Piece::Knight => PieceLen::One,
        //     Piece::Bishop | Piece::Rook | Piece::Queen => PieceLen::Infinity,
        //     Piece::King => PieceLen::One,
        // };
    }
}

fn offset_vec_pos(vec: (i8, i8), offset: (i8, i8)) -> (i8, i8) {
    (vec.0 * offset.0, vec.1 * offset.1)
}

#[derive(Debug)]
enum PieceLen {
    One,
    Two,
    Infinity,
}
