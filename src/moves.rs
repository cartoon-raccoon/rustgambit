use core::ops::{Deref, DerefMut};
use core::convert::AsRef;
use std::vec::IntoIter;

#[derive(Clone)]
pub struct MoveSet {
    moves: Vec<Move>
}

impl MoveSet {
    pub fn len(&self) -> usize {
        self.moves.len()
    }

    pub fn iter(&self) -> MoveSetIter<'_> {
        MoveSetIter {
            idx: 0,
            inner: &self,
        }
    }
}

impl IntoIterator for MoveSet {
    type Item = Move;
    type IntoIter = MoveSetIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        MoveSetIntoIter {
            inner: self.moves.into_iter()
        }
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

    #[inline]
    fn as_ref(&self) -> &[Move] {
        &self.moves
    }
}

impl<'a> AsMut<[Move]> for MoveSet {

    #[inline]
    fn as_mut(&mut self) -> &mut [Move] {
        &mut self.moves
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MoveSetIter<'a> {
    idx: usize,
    inner: &'a [Move],
}

impl<'a> Iterator for MoveSetIter<'a> {
    type Item = &'a Move;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx + 1 == self.inner.len() {
            return None
        } else {
            self.idx += 1;
            return Some(&self.inner[self.idx])
        }
    }
}

#[derive(Debug, Clone)]
pub struct MoveSetIntoIter {
    inner: IntoIter<Move>,
}

impl Iterator for MoveSetIntoIter {
    type Item = Move;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Move {
    target: (u8, u8),
    origin: (u8, u8),
}

//todo: implement Display and Debug