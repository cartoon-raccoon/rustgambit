use std::fmt;
use std::error::Error;
use std::ops::{Index, IndexMut};

use crate::pieces::*;
use crate::moves::{Move, MoveMarker};

pub struct Board {
    pub board: [[PieceType; 8]; 8],
    turn: Colour,
    // indicates which king is being checked
    check: Option<Colour>,
    white_cap: Vec<PieceType>,
    black_cap: Vec<PieceType>,
}

impl Board {
    pub fn empty() -> Self {
        Board{
            board: [[PieceType::Empty; 8]; 8],
            turn: Colour::White,
            check: None,
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
        self.board[0][0] = PieceType::Rook(Rook::config(0, 0, White));

        self.board[0][7] = PieceType::Rook(Rook::config(0, 7, White));

        self.board[7][0] = PieceType::Rook(Rook::config(7, 0, Black));

        self.board[7][7] = PieceType::Rook(Rook::config(7, 7, Black));

        //initializing knights
        self.board[0][1] = PieceType::Knight(Knight::config(0, 1, White));

        self.board[0][6] = PieceType::Knight(Knight::config(0, 6, White));

        self.board[7][1] = PieceType::Knight(Knight::config(7, 1, Black));

        self.board[7][6] = PieceType::Knight(Knight::config(7, 6, Black));

        //initializing bishops
        self.board[0][2] = PieceType::Bishop(Bishop::config(0, 2, White));

        self.board[0][5] = PieceType::Bishop(Bishop::config(0, 5, White));

        self.board[7][2] = PieceType::Bishop(Bishop::config(7, 2, Black));

        self.board[7][5] = PieceType::Bishop(Bishop::config(7, 5, Black));

        //initializing queens and kings
        self.board[0][3] = PieceType::Queen(Queen::config(0, 3, White));

        self.board[0][4] = PieceType::King(King::config(0, 4, White));

        self.board[7][3] = PieceType::Queen(Queen::config(7, 3, Black));

        self.board[7][4] = PieceType::King(King::config(7, 4, Black));

        //initializing pawns
        for i in 0..8 {
            self.board[1][i] = PieceType::Pawn(
                Pawn::config(1, i, White)
            );

            self.board[6][i] = PieceType::Pawn(
                Pawn::config(6, i, Black)
            );
        }

        if !is_empty {
            for i in 2..5 {
                for j in 0..7 {
                    self.board[i][j] = PieceType::Empty;
                }
            }
        }
    }

    pub fn move_piece(&mut self, mov: Move) -> Result<(), GameError> {
        //moving the piece at origin to a temp variable
        let temp = self.board[mov.origin().0][mov.origin().1];
        let targ = self.board[mov.target().1][mov.target().1];

        //checking that the origin is actually not empty
        if let Some(colour) = temp.colour() {
            if colour != self.turn {
                return Err(GameError::WrongTurn)
            }
        } else { //colour returned None, so must be empty
            assert!(temp.is_empty());
            return Err(GameError::EmptySpace)
        }

        //check if target is empty; if not, add target to captures
        if !targ.is_empty() {
            self.add_to_captures(self.turn, targ)
        }

        //making the actual move
        self.board[mov.origin().0][mov.origin().1] = PieceType::Empty;
        self.board[mov.target().0][mov.target().1] = temp;

        self.log_move(temp, mov);

        Ok(())
    }

    pub fn add_to_captures(&mut self, colour: Colour, piece: PieceType) {
        use Colour::*;

        assert!(!piece.is_empty(), "Empty piece given");
        assert!(piece.colour() != Some(self.turn), "Colour mismatch");

        match colour {
            White => {
                self.white_cap.push(piece)
            }
            Black => {
                self.black_cap.push(piece)
            }
        }
    }

    pub fn log_move(&mut self, piece: PieceType, _mov: Move) {
        assert!(!piece.is_empty());
        unimplemented!("logging moves unimplemented")
    }

    pub fn flip_turn(&mut self) {
        if self.turn == Colour::Black {
            self.turn = Colour::White
        } else {
            self.turn = Colour::Black
        }
    }

    pub fn eval_gamestate(&mut self) -> GameState {
        let _kingpos = self.find_king(self.turn);

        for pos in self.enum_opposite_pieces() {
            let (row, col) = pos.as_tuple();

            if self[row][col]
                .inner().unwrap()
                .evaluate_moves(&self)
                .is_checking() {
                self.check = Some(self.turn);
                return GameState::Check(self.turn);
                //todo: work this outtt
            }
        }

        unimplemented!()
    }

    fn enum_opposite_pieces(&self) -> Vec<Position> {

        let mut positions = Vec::new();

        for i in 0..8 {
            for j in 0..8 {
                if let Some(colour) = self[i][j].colour() {
                    if colour != self.turn {
                        positions.push(self[i][j]
                            .inner()
                            // Unwrap should never panic because colour is not None
                            .unwrap()
                            .position())
                    }
                }
            }
        }

        positions
    }

    pub(crate) fn find_king(&self, c: Colour) -> Position {

        for i in 0..8 {
            for j in 0..8 {
                if let PieceType::King(k) = self[i][j] {
                    if k.colour() == c {
                        return k.position()
                    }
                }
            }
        }

        panic!("Could not find king of colour {:?}", c)
    }

    #[inline]
    pub fn is_in_check(&self) -> bool {
        if let Some(_) = self.check {
            true
        } else {
            false
        }
    }

    #[inline]
    pub fn black(&self) -> &[PieceType] {
        &self.black_cap
    }

    #[inline]
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
            for row in col.iter() {
                buf.push(row.as_char());
                buf.push('|');
            }
            buf.push('\n');
        }

        write!(f, "{}", buf)
    }
}

impl Index<usize> for Board {
    type Output = [PieceType; 8];

    #[inline]
    fn index(&self, idx: usize) -> &Self::Output {
        &self.board[idx]
    }
}

impl IndexMut<usize> for Board {

    #[inline]
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.board[idx]
    }
}

/// Denotes the state of the game.
#[derive(Clone, Copy, PartialEq)]
pub enum GameState {
    /// The gameboard is in a neutral state.
    /// No one is in check or checkmate, and there are available moves.
    None,
    /// The king of the returned colour is in check.
    Check(Colour),
    /// The game is in a stalemate.
    Stalemate,
    /// The king of the returned colour is in checkmate.
    /// The game is over.
    Checkmate(Colour),
}

#[non_exhaustive]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum GameError {
    InvalidMove,
    WrongTurn,
    EmptySpace,
}

impl Error for GameError {}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidMove => {
                write!(f, "Invalid move for piece")
            }
            Self::WrongTurn => {
                write!(f, "Wrong colour to make turn")
            }
            Self::EmptySpace => {
                write!(f, "Space on board is empty")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board_display() {
        let board = Board::init();

        let test_board = 
"|r|n|b|q|k|b|n|r|
|p|p|p|p|p|p|p|p|
| | | | | | | | |
| | | | | | | | |
| | | | | | | | |
| | | | | | | | |
|P|P|P|P|P|P|P|P|
|R|N|B|Q|K|B|N|R|
";

        assert_eq!(board.to_string(), test_board)
    }

    #[test]
    fn test_find_king() {
        let board = Board::init();
        let black_k = board.find_king(Colour::Black);
        let white_k = board.find_king(Colour::White);

        assert_eq!(black_k, (7, 4));
        assert_eq!(white_k, (0, 4));
    }
}