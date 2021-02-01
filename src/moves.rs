use core::ops::{Deref, DerefMut};
use core::convert::AsRef;
use std::vec::IntoIter;

/// Allows a generic type to be marked as a Move.
pub trait MoveMarker {
    fn target(&self) -> (usize, usize);
    fn origin(&self) -> (usize, usize);
}

/// Contains the set of Moves that a particular piece can take.
/// 
/// Returned by `Piece::evaluate_moves`.
#[derive(Clone)]
pub struct MoveSet {
    is_checking: bool,
    moves: Vec<Move>
}

impl MoveSet {

    /// Constructs a MoveSet from a type that implements IntoIterator
    /// and its associated Item implements MoveMarker.
    pub fn from<T: IntoIterator>(iter: T) -> Self
    where T::Item: MoveMarker {
        let mut moves = Vec::new();

        for item in iter {
            moves.push(Move::construct(item.target(), item.origin()));
        }

        MoveSet {
            is_checking: false,
            moves: moves
        }
    }

    pub fn check() -> Self {
        MoveSet {
            is_checking: true,
            moves: Vec::new(),
        }
    }

    /// Returns the number of Moves in the MoveSet
    #[inline]
    pub fn len(&self) -> usize {
        self.moves.len()
    }

    /// Returns whether the MoveSet is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.moves.is_empty()
    }

    /// Returns whether the piece that generated the MoveSet is in check.
    #[inline]
    pub fn is_checking(&self) -> bool {
        self.is_checking
    }

    /// Returns an iterator over the contents of the MoveSet
    /// 
    /// Borrows the Moves immutably, there is no mutable implementation.
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

impl AsRef<[Move]> for MoveSet {

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
    target: (usize, usize),
    origin: (usize, usize),
}

impl Move {
    pub(crate) fn construct(t: (usize, usize), o: (usize, usize)) -> Self {
        Move {
            target: t,
            origin: o,
        }
    }
}

impl MoveMarker for Move {
    fn target(&self) -> (usize, usize) {
        self.target
    }

    fn origin(&self) -> (usize, usize) {
        self.origin
    }
}

//todo: implement Display and Debug