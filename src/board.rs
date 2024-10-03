use std::fmt::Display;

use crate::piece::File;
use crate::piece::Piece;
use crate::piece::PieceColor;
use crate::piece::PieceLen;
use crate::piece::Rank;
use crate::GameState;
use crate::GameTurn;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Board {
    pub(crate) data: [PieceColor; 64],
}

impl Board {
    pub(crate) fn new(fen: Option<String>) -> Self {
        if let Some(_fen) = fen {
            todo!();
        } else {
            Self {
                data: [PieceColor::Empty; 64],
            }
        }
    }

    fn is_type_check(
        &self,
        move_set: Vec<(i8, i8)>,
        turn: &GameTurn,
        king_pos: &(u8, u8),
    ) -> Vec<PieceColor> {
        Piece::Queen
            .collect_along_dirs_lists(
                self,
                turn,
                move_set.into_iter(),
                king_pos,
                &PieceLen::Infinity,
            )
            .into_iter()
            .filter_map(|p| {
                let last = *p.last()?;
                match self.get_piece_at(&last) {
                    PieceColor::White(_piece) => match turn {
                        GameTurn::White => None,
                        GameTurn::Black => Some(last),
                    },
                    PieceColor::Black(_piece) => match turn {
                        GameTurn::White => Some(last),
                        GameTurn::Black => None,
                    },
                    PieceColor::Empty => None,
                }
            })
            .map(|p| self.get_piece_at(&p))
            .collect()
    }

    pub(crate) fn is_check(&self, turn: &GameTurn, king_pos: &(u8, u8)) -> bool {
        let bishop_moves = Piece::Bishop.get_move_set(turn);
        let q = self.is_type_check(bishop_moves, turn, king_pos);
        if q.into_iter().any(|p| matches!(p.get_piece().unwrap(), Piece::Bishop | Piece::Queen)) {
            return true;
        }
        let rook_moves = Piece::Rook.get_move_set(turn);
        let q = self.is_type_check(rook_moves, turn, king_pos);
        if q.into_iter().any(|p| matches!(p.get_piece().unwrap(), Piece::Rook | Piece::Queen)) {
            return true;
        }
        let knight_moves = Piece::Knight.get_move_set(turn);
        let q = self.is_type_check(knight_moves, turn, king_pos);
        if q.into_iter().any(|p| matches!(p.get_piece().unwrap(), Piece::Knight)) {
            return true;
        }
        let pawn_moves = Piece::Pawn(true).get_capture_set(!turn);
        let q = self.is_type_check(pawn_moves, turn, king_pos);
        if q.into_iter().any(|p| matches!(p.get_piece().unwrap(), Piece::Pawn(_))) {
            return true;
        }

        false

    }

    // pub(crate) fn is_pos_check(&self, turn: &GameTurn, king_pos: &(u8, u8)) -> bool {
    //     println!("Checking... {:?} for {:?}", king_pos, turn);
    //     let q = Piece::Queen
    //         .get_possible_moves(self, !turn, king_pos)
    //         .into_iter()
    //         .map(|pos| (self.get_piece_at(&pos), pos))
    //         .filter_map(|(p, pos)| match p {
    //             PieceColor::White(piece) => match turn {
    //                 GameTurn::White => None,
    //                 GameTurn::Black => Some((piece, pos)),
    //             },
    //             PieceColor::Black(piece) => match turn {
    //                 GameTurn::White => Some((piece, pos)),
    //                 GameTurn::Black => None,
    //             },
    //             PieceColor::Empty => None,
    //         })
    //         .map(|(p, pos)| p.get_possible_moves(self, turn, &pos))
    //         .any(|m| {
    //             println!("{m:?}, king pos: {king_pos:?}");
    //             // view_pos(&m);
    //             m.contains(king_pos)
    //         });
    //     if q {
    //         return q;
    //     }
    //     Piece::Knight
    //         .get_possible_moves(self, !turn, king_pos)
    //         .into_iter()
    //         .map(|pos| (self.get_piece_at(&pos), pos))
    //         .filter_map(|(p, pos)| match p {
    //             PieceColor::White(piece) | PieceColor::Black(piece) => match piece {
    //                 Piece::Knight => Some((piece, pos)),
    //                 _ => None,
    //             },
    //             PieceColor::Empty => None,
    //         })
    //         .map(|(p, pos)| p.get_possible_moves(self, turn, &pos))
    //         .any(|m| {
    //             println!("{m:?}, king pos: {king_pos:?}");
    //             // view_pos(&m);
    //             m.contains(king_pos)
    //         })
    // }

    pub(crate) fn get_piece_at(&self, pos: &(u8, u8)) -> PieceColor {
        let (x, y) = pos;
        self.data[*y as usize * 8 + *x as usize]
    }

    pub(crate) fn set_piece_at(&mut self, pos: &(u8, u8), piece_color: PieceColor) {
        let (x, y) = pos;
        self.data[*y as usize * 8 + *x as usize] = piece_color;
    }

    pub(crate) fn check_promotion(&self, pos: &(u8, u8), turn: &GameTurn) -> Option<GameState> {
        match turn {
            GameTurn::White => {
                if pos.1 == 0 {
                    return Some(GameState::Promotion((
                        Rank::try_from(pos.0).ok()?,
                        File::try_from(pos.1).ok()?,
                    )));
                }
            }
            GameTurn::Black => {
                if pos.1 == 7 {
                    return Some(GameState::Promotion((
                        Rank::try_from(pos.0).ok()?,
                        File::try_from(pos.1).ok()?,
                    )));
                }
            }
        }
        Some(GameState::InProgress)
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = String::new();
        for (i, p) in self.data.iter().enumerate() {
            res.push_str(format!("{}", p).as_str());
            if (i + 1) % 8 == 0 {
                res.push('\n');
            }
        }
        write!(f, "{}", res)
    }
}

impl Default for Board {
    fn default() -> Self {
        let mut board = Board {
            data: [PieceColor::Empty; 64],
        };

        board.set_piece_at(&(0, 0), PieceColor::Black(Piece::Rook));
        board.set_piece_at(&(1, 0), PieceColor::Black(Piece::Knight));
        board.set_piece_at(&(2, 0), PieceColor::Black(Piece::Bishop));
        board.set_piece_at(&(3, 0), PieceColor::Black(Piece::Queen));
        board.set_piece_at(&(4, 0), PieceColor::Black(Piece::King));
        board.set_piece_at(&(5, 0), PieceColor::Black(Piece::Bishop));
        board.set_piece_at(&(6, 0), PieceColor::Black(Piece::Knight));
        board.set_piece_at(&(7, 0), PieceColor::Black(Piece::Rook));

        for i in 0..8 {
            board.set_piece_at(&(i, 1), PieceColor::Black(Piece::Pawn(true)));
        }

        board.set_piece_at(&(0, 7), PieceColor::White(Piece::Rook));
        board.set_piece_at(&(1, 7), PieceColor::White(Piece::Knight));
        board.set_piece_at(&(2, 7), PieceColor::White(Piece::Bishop));
        board.set_piece_at(&(3, 7), PieceColor::White(Piece::Queen));
        board.set_piece_at(&(4, 7), PieceColor::White(Piece::King));
        board.set_piece_at(&(5, 7), PieceColor::White(Piece::Bishop));
        board.set_piece_at(&(6, 7), PieceColor::White(Piece::Knight));
        board.set_piece_at(&(7, 7), PieceColor::White(Piece::Rook));

        for i in 0..8 {
            board.set_piece_at(&(i, 6), PieceColor::White(Piece::Pawn(true)));
        }

        board
    }
}

pub(crate) fn view_pos(positions: &[(u8, u8)]) {
    let mut board = Board::new(None);
    for pos in positions {
        board.set_piece_at(pos, PieceColor::White(Piece::Queen));
    }
    println!("{}", board);
}
