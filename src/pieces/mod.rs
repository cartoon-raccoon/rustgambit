pub mod pawn;
pub mod rook;
pub mod knight;

use crate::moves::MoveSet;
use crate::board::Board;

pub use pawn::Pawn;
pub use rook::Rook;
pub use knight::Knight;

pub trait Piece {
    fn move_piece(&self, board: &mut Board);
    fn evaluate_moves(&self, board: &mut Board) -> MoveSet;
}

#[derive(Clone, Copy)]
pub enum PieceType {
    Pawn{
        piece: Pawn,
        colour: Colour,
        pos: Position,
    },
    Rook {
        piece: Rook,
        colour: Colour,
        pos: Position,
    },
    Knight {
        piece: Knight,
        colour: Colour,
        pos: Position,
    },
    Bishop {
        colour: Colour,
        pos: Position,
    },
    Queen {
        colour: Colour,
        pos: Position,
    },
    King {
        colour: Colour,
        pos: Position,
    },
    Empty,
}

#[derive(Clone, Copy)]
pub enum Colour {
    Black,
    White,
}

#[derive(Clone, Copy)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}