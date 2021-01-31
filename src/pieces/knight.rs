use crate::pieces::Piece;
use crate::moves::MoveSet;
use crate::board::Board;
use super::*;

pub const POINTS: u8 = 3;

#[derive(Clone, Copy, PartialEq)]
pub struct Knight {
    colour: Colour,
    pos: Position,
}

impl Piece for Knight {
    fn config(row: usize, col: usize, colour: Colour) -> Self {
        let pos = Position {row: row, col: col};

        Self {
            colour: colour,
            pos: pos,
        }
    }

    #[inline]
    fn colour(&self) -> Colour {
        self.colour
    }
    
    fn evaluate_moves(&self, board: &mut Board) -> MoveSet {
        unimplemented!()
    }
}