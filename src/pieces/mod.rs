pub mod pawn;
pub mod rook;
pub mod knight;
pub mod bishop;
pub mod queen;
pub mod king;

use crate::moves::MoveSet;
use crate::board::Board;

pub use pawn::Pawn;
pub use rook::Rook;
pub use knight::Knight;
pub use bishop::Bishop;
pub use queen::Queen;
pub use king::King;

pub trait Piece {
    //fn move_piece(&self, board: &mut Board);
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
        piece: Bishop,
        colour: Colour,
        pos: Position,
    },
    Queen {
        piece: Queen,
        colour: Colour,
        pos: Position,
    },
    King {
        piece: King,
        colour: Colour,
        pos: Position,
    },
    Empty,
}

impl PieceType {

    #[inline]
    pub fn colour(&self) -> Option<Colour> {
        use PieceType::*;

        match self {
            Empty => None,
            Pawn {colour,..} => Some(*colour),
            Knight {colour,..} => Some(*colour),
            Bishop {colour,..} => Some(*colour),
            Rook {colour,..} => Some(*colour),
            Queen {colour,..} => Some(*colour),
            King {colour,..} => Some(*colour),
        }
    }

    pub fn is_empty(&self) -> bool {
        if let PieceType::Empty = self {
            return true
        }
        false
    }
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