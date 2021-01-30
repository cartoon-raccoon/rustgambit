use crate::pieces::Piece;
use crate::moves::MoveSet;
use crate::board::Board;

pub const POINTS: u8 = 5;

#[derive(Clone, Copy, Default, PartialEq)]
pub struct Rook;

impl Piece for Rook {
    fn move_piece(&self, board: &mut Board) {

    }

    fn evaluate_moves(&self, board: &mut Board) -> MoveSet {
        unimplemented!()
    }
}