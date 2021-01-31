use crate::pieces::Piece;
use crate::moves::{MoveSet, Move};
use crate::board::Board;
use super::*;

pub const POINTS: u8 = 1;

#[derive(Copy, Clone, PartialEq)]
pub struct Pawn {
    colour: Colour,
    pos: Position,
}

impl Piece for Pawn {
    fn config(row: usize, col: usize, colour: Colour) -> Self {
        let pos = Position {row: row, col: col};

        Self {
            colour: colour,
            pos: pos,
        }
    }

    #[inline]
    fn colour(&self) -> Colour {
        self.colour
    }

    fn evaluate_moves(&self, board: &Board) -> MoveSet {
        use Colour::*;

        let o = self.pos.as_tuple();
        let mut moves = Vec::new();

        match self.colour {
            White => {
                if board[o.0 + 1][o.1].is_empty() {
                    moves.push(Move::construct(o, (o.0 + 1, o.1)))
                }
                if !board[o.0 + 1][o.1 + 1].is_empty() {
                    moves.push(Move::construct(o, (o.0 + 1, o.1 + 1)))
                }
                if !board[o.0 + 1][o.1 - 1].is_empty() {
                    moves.push(Move::construct(o, (o.0 + 1, o.1 - 1)))
                }
            }

            Black => {
                if board[o.0 - 1][o.1].is_empty() {
                    moves.push(Move::construct(o, (o.0 + 1, o.1)))
                }
                if !board[o.0 - 1][o.1 + 1].is_empty() {
                    moves.push(Move::construct(o, (o.0 + 1, o.1 + 1)))
                }
                if !board[o.0 - 1][o.1 - 1].is_empty() {
                    moves.push(Move::construct(o, (o.0 + 1, o.1 - 1)))
                }
            }
        }
        
        MoveSet::from(moves)
    }
}