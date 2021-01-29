pub mod pawn;

use crate::moves::MoveSet;

pub trait Piece {
    fn move_piece();
    fn evaluate_moves() -> MoveSet;
}

pub enum PieceType {
    Pawn{
        colour: Colour,
        data: PieceAttr
    }
}

pub enum Colour {
    Black,
    White,
}

pub struct PieceAttr {

}