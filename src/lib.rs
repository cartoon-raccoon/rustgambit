pub mod pieces; 

pub struct Board {
    
}

pub enum Piece {
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
