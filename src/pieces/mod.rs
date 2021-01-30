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

#[derive(Clone, Copy, PartialEq)]
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

impl PieceType {
    pub fn as_char(&self) -> char {
        use PieceType::*;
        use Colour::*;

        match self {
            Pawn {colour,..} => {
                match colour {
                    White => 'P',
                    Black => 'p',
                }
            }
            Rook {colour,..} => {
                match colour {
                    White => 'R',
                    Black => 'r',
                }
            }
            Knight {colour,..} => {
                match colour {
                    White => 'N',
                    Black => 'n',
                }
            }
            Bishop {colour,..} => {
                match colour {
                    White => 'B',
                    Black => 'b',
                }
            }
            Queen {colour,..} => {
                match colour {
                    White => 'Q',
                    Black => 'q',
                }
            }
            King {colour,..} => {
                match colour {
                    White => 'K',
                    Black => 'k',
                }
            }
            Empty => ' '
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Colour {
    Black,
    White,
}

#[derive(Clone, Copy, PartialEq)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}