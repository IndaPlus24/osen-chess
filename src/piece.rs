use std::usize;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PieceColor {
    White(Piece),
    Black(Piece),
    Empty,
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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Piece {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}
