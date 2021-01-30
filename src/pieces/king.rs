use crate::pieces::Piece;
use crate::moves::MoveSet;
use crate::board::Board;

pub const POINTS: u8 = 3;

#[derive(Clone, Copy, Default, PartialEq)]
pub struct King;

impl Piece for King {
    fn move_piece(&self, board: &mut Board) {

    }

    fn evaluate_moves(&self, board: &mut Board) -> MoveSet {
        unimplemented!()
    }
}