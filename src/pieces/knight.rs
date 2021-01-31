use crate::pieces::Piece;
use crate::moves::MoveSet;
use crate::board::Board;

pub const POINTS: u8 = 3;

#[derive(Clone, Copy, Default, PartialEq)]
pub struct Knight;

impl Piece for Knight {
    fn evaluate_moves(&self, board: &mut Board) -> MoveSet {
        unimplemented!()
    }
}