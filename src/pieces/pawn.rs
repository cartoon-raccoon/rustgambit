use crate::pieces::Piece;
use crate::moves::MoveSet;
use crate::board::Board;

pub const POINTS: u8 = 1;

#[derive(Default, Copy, Clone, PartialEq)]
pub struct Pawn;

impl Piece for Pawn {
    fn evaluate_moves(&self, board: &mut Board) -> MoveSet {
        unimplemented!()
    }
}