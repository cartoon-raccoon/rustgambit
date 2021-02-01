use crate::pieces::Piece;
use crate::moves::MoveSet;
use crate::board::Board;
use super::*;

pub const POINTS: u8 = 5;

#[derive(Clone, Copy, PartialEq)]
pub struct Rook {
    colour: Colour,
    pos: Position,
}

impl Rook {
    pub fn config(row: usize, col: usize, colour: Colour) -> Self {
        let pos = Position {row: row, col: col};

        Self {
            colour: colour,
            pos: pos,
        }
    }
}

impl Piece for Rook {
    #[inline]
    fn colour(&self) -> Colour {
        self.colour
    }

    #[inline]
    fn position(&self) -> Position {
        self.pos
    }
    
    fn evaluate_moves(&self, board: &Board) -> MoveSet {
        unimplemented!()
    }
}