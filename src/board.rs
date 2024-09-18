use std::fmt::Display;
use std::usize;

use crate::piece::File;
use crate::piece::Piece::*;
use crate::piece::PieceColor;
use crate::piece::Rank;

#[derive(Debug)]
pub struct Board {
    data: [[PieceColor; 8]; 8],
}

impl Board {
    pub fn get_piece_at(&self, pos: &(File, Rank)) -> PieceColor {
        let (f, r) = pos;
        self.data[usize::from(f)][usize::from(r)]
    }

    pub fn set_piece_at(&mut self, pos: &(File, Rank), piece_color: PieceColor) {
        let (f, r) = pos;
        self.data[usize::from(f)][usize::from(r)] = piece_color;
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { 
        let mut res = String::new();
        for file in self.data {
            for rank in file {
                res.push_str(format!("{} ", rank).as_str());
            }
            res.push('\n');
        }
        write!(f, "{}", res)
    }
}

impl Default for Board {
    fn default() -> Self {
        let mut data: [[PieceColor; 8]; 8] = [[PieceColor::Empty; 8]; 8];
        data[0] = (
            PieceColor::Black(Rook),
            PieceColor::Black(Knight),
            PieceColor::Black(Bishop),
            PieceColor::Black(Queen),
            PieceColor::Black(King),
            PieceColor::Black(Bishop),
            PieceColor::Black(Knight),
            PieceColor::Black(Rook),
        )
            .into();
        data[1] = [PieceColor::Black(Pawn); 8];
        data[2] = [PieceColor::Empty; 8];
        data[3] = [PieceColor::Empty; 8];
        data[4] = [PieceColor::Empty; 8];
        data[5] = [PieceColor::Empty; 8];
        data[6] = [PieceColor::White(Pawn); 8];
        data[7] = (
            PieceColor::White(Rook),
            PieceColor::White(Knight),
            PieceColor::White(Bishop),
            PieceColor::White(Queen),
            PieceColor::White(King),
            PieceColor::White(Bishop),
            PieceColor::White(Knight),
            PieceColor::White(Rook),
        )
            .into();
        Board { data }
    }
}
