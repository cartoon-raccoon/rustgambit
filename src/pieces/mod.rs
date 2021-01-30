pub mod pawn;
pub mod knight;

use crate::moves::MoveSet;
use crate::board::Board;
use pawn::Pawn;

pub trait Piece {
    fn move_piece(&self, board: &mut Board);
    fn evaluate_moves(&self, board: &mut Board) -> MoveSet;
}

pub enum PieceType {
    Pawn{
        piece: Pawn,
        colour: Colour,
        pos: Position,
    },
    Rook {
        colour: Colour,
        pos: Position,
    },
    Knight {
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
    }
}

pub enum Colour {
    Black,
    White,
}

pub struct Position {
    pub col: usize,
    pub row: usize,
}