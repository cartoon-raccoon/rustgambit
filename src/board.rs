use std::fmt;

use crate::pieces::*;

pub struct Board {
    pub board: [[PieceType; 8]; 8],
    white_cap: Vec<PieceType>,
    black_cap: Vec<PieceType>,
}

impl Board {
    pub fn empty() -> Self {
        Board{
            board: [[PieceType::Empty; 8]; 8],
            white_cap: Vec::new(),
            black_cap: Vec::new(),
        }
    }

    pub fn init() -> Self {
        let mut board = Board::empty();
        board.reset();

        board
    }

    pub fn reset(&mut self) {
        use Colour::*;

        let is_empty = self.is_empty();

        //initializing rooks
        self.board[0][0] = PieceType::Rook {
            piece: Rook,
            colour: White, 
            pos: Position {row: 0, col: 0}
        };

        self.board[0][7] = PieceType::Rook {
            piece: Rook,
            colour: White,
            pos: Position {row: 0, col: 7}
        };

        self.board[7][0] = PieceType::Rook {
            piece: Rook,
            colour: Black,
            pos: Position {row: 7, col: 0}
        };

        self.board[7][7] = PieceType::Rook {
            piece: Rook,
            colour: Black,
            pos: Position {row: 7, col: 7}
        };

        //initializing knights
        self.board[0][1] = PieceType::Knight {
            piece: Knight,
            colour: White,
            pos: Position {row: 0, col: 1},
        };

        self.board[0][6] = PieceType::Knight {
            piece: Knight,
            colour: White,
            pos: Position {row: 0, col: 6},
        };

        self.board[7][1] = PieceType::Knight {
            piece: Knight,
            colour: Black,
            pos: Position {row: 7, col: 1},
        };

        self.board[7][6] = PieceType::Knight {
            piece: Knight,
            colour: Black,
            pos: Position {row: 7, col: 6},
        };

        //initializing bishops
        self.board[0][2] = PieceType::Bishop {
            piece: Bishop,
            colour: White,
            pos: Position {row: 0, col: 2},
        };

        self.board[7][2] = PieceType::Bishop {
            piece: Bishop,
            colour: Black,
            pos: Position {row: 7, col: 2},
        };

        self.board[0][5] = PieceType::Bishop {
            piece: Bishop,
            colour: White,
            pos: Position {row: 0, col: 5},
        };

        self.board[7][5] = PieceType::Bishop {
            piece: Bishop,
            colour: Black,
            pos: Position {row: 7, col: 5},
        };

        //initializing queens and kings
        self.board[0][3] = PieceType::Queen {
            piece: Queen,
            colour: White,
            pos: Position {row: 0, col: 3},
        };

        self.board[0][4] = PieceType::King {
            piece: King,
            colour: White,
            pos: Position {row: 0, col: 4}
        };

        self.board[7][3] = PieceType::Queen {
            piece: Queen,
            colour: Black,
            pos: Position {row: 7, col: 3},
        };

        self.board[7][4] = PieceType::King {
            piece: King,
            colour: Black,
            pos: Position {row: 7, col: 4}
        };

        //initializing pawns
        for i in 0..7 {
            self.board[1][i] = PieceType::Pawn {
                piece: Pawn,
                colour: White,
                pos: Position {row: 1, col: i}
            };

            self.board[6][i] = PieceType::Pawn {
                piece: Pawn,
                colour: Black,
                pos: Position {row: 6, col: i}
            };
        }

        if !is_empty {
            for i in 2..5 {
                for j in 0..7 {
                    self.board[i][j] = PieceType::Empty;
                }
            }
        }
    }

    pub fn black(&self) -> &[PieceType] {
        &self.black_cap
    }

    pub fn white(&self) -> &[PieceType] {
        &self.white_cap
    }

    pub fn is_empty(&self) -> bool {
        self.board.iter()
            .all(|row| row.iter().all(
                |col| *col == PieceType::Empty
            )
        )
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();

        for col in self.board.iter().rev() {
            buf.push('|');
            for row in col.iter().rev() {
                buf.push(row.as_char());
                buf.push('|');
            }
            buf.push('\n');
        }

        write!(f, "{}", buf)
    }
}