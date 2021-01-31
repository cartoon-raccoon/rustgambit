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

/// Defines the basic moves a piece can take.
pub trait Piece {
    fn config(row: usize, col: usize, colour: Colour) -> Self;
    fn colour(&self) -> Colour;
    fn evaluate_moves(&self, board: &Board) -> MoveSet;
}

/// Defines the type of Piece.
#[derive(Clone, Copy, PartialEq)]
pub enum PieceType {
    Pawn(Pawn),
    Rook(Rook),
    Knight(Knight),
    Bishop(Bishop),
    Queen(Queen),
    King(King),
    Empty,
}

impl PieceType {

    /// Returns the colour of the PieceType.
    #[inline]
    pub fn colour(&self) -> Option<Colour> {
        use PieceType::*;

        match self {
            Empty => None,
            Pawn(p) => Some(p.colour()),
            Knight(n) => Some(n.colour()),
            Bishop(b) => Some(b.colour()),
            Rook(r) => Some(r.colour()),
            Queen(q)=> Some(q.colour()),
            King(k) => Some(k.colour()),
        }
    }

    /// Returns true if the PieceType is Empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        if let PieceType::Empty = self {
            return true
        }
        false
    }

    pub fn is_king(&self) -> bool {
        if let PieceType::King(_) = self {
            return true
        }
        false
    }

    /// Returns the corresponding notation for the piece.
    pub fn as_char(&self) -> char {
        use PieceType::*;
        use Colour::*;

        match self {
            Pawn(p) => {
                match p.colour() {
                    White => 'P',
                    Black => 'p',
                }
            }
            Rook(p) => {
                match p.colour() {
                    White => 'R',
                    Black => 'r',
                }
            }
            Knight(p) => {
                match p.colour() {
                    White => 'N',
                    Black => 'n',
                }
            }
            Bishop(p) => {
                match p.colour() {
                    White => 'B',
                    Black => 'b',
                }
            }
            Queen(p) => {
                match p.colour() {
                    White => 'Q',
                    Black => 'q',
                }
            }
            King(p) => {
                match p.colour() {
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

impl Position {
    pub fn as_tuple(&self) -> (usize, usize) {
        (self.row, self.col)
    }
}