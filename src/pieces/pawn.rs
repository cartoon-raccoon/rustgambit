use crate::pieces::Piece;
use crate::moves::MoveSet;
use crate::board::Board;

pub const POINTS: u8 = 1;

pub struct Pawn;

impl Piece for Pawn {
    fn move_piece(&self, board: &mut Board) {

    }

    fn evaluate_moves(&self, board: &mut Board) -> MoveSet {
        unimplemented!()
    }
}