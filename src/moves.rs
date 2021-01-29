use std::ops::{Deref, DerefMut};

pub struct MoveSet {
    moves: Vec<Move>
}

impl Iterator for MoveSet {
    type Item = Move;

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}

impl Deref for MoveSet {
    type Target = MoveSetIter<'a>;


}

impl DerefMut for MoveSet {
    type Target = MoveSetIter<'a>;

    fn deref_mut(&mut self) -> &mut Self::Target {
        
    }
}

struct MoveSetIter<'a> {
    inner: &'a mut MoveSet
}

pub struct Move {
    target: (u8, u8),
    origin: (u8, u8),
}