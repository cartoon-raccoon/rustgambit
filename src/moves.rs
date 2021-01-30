use std::ops::{Deref, DerefMut};
use std::convert::AsRef;

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
    type Target = [Move];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.moves
    }
}

impl DerefMut for MoveSet {

    #[inline]
    fn deref_mut(&mut self) -> &mut [Move] {
        &mut self.moves
    }
}

impl<'a> AsRef<[Move]> for MoveSet {
    fn as_ref(&self) -> &[Move] {
        &self.moves
    }
}

struct MoveSetIter<'a> {
    inner: &'a mut MoveSet
}

pub struct Move {
    target: (u8, u8),
    origin: (u8, u8),
}