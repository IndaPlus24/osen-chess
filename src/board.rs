use crate::piece::File;
use crate::piece::Piece::*;
use crate::piece::PieceColor;
use crate::piece::Rank;

#[derive(Debug)]
pub struct Board {
    data: [[PieceColor; 8]; 8],
}

impl Board {
    pub fn get_piece_at(&self, pos: (File, Rank)) -> PieceColor {
        let f: usize = pos.0.into();
        let s: usize = pos.1.into();
        self.data[f][s]
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
