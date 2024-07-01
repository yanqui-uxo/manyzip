//! Zip an arbitrary number of iterables
//!
//! (Note: the term "iterable" is used to refer to any instances of types that implement [`IntoIterator`])

#![no_std]

extern crate alloc;
use alloc::boxed::Box;
use alloc::vec::Vec;

/// Iterator that zips an arbitrary number of iterables into [`Vec`]s
///
/// Created by [manyzip], should not be used directly
#[derive(Debug)]
pub struct Manyzip<T: Iterator>(Box<[T]>);

impl<T: Iterator> Iterator for Manyzip<T> {
    type Item = Vec<T::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.is_empty() {
            return None;
        }
        self.0.iter_mut().map(Iterator::next).collect()
    }
}

/// Returns an iterator that zips all iterables provided into [`Vec`]s
///
/// Yields `None` if any of the iterables yield `None`
pub fn manyzip<T: IntoIterator>(iterators: T) -> Manyzip<<T::Item as IntoIterator>::IntoIter>
where
    T::Item: IntoIterator,
{
    Manyzip(iterators.into_iter().map(IntoIterator::into_iter).collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn basic() {
        let test1: Vec<Vec<i32>> =
            manyzip(vec![vec![1, 2, 3], vec![4, 5], vec![6, 7, 8]]).collect();
        let test2: Vec<Vec<i32>> = vec![vec![1, 4, 6], vec![2, 5, 7]];
        assert_eq!(test1, test2);
    }

    #[test]
    fn empty_inner() {
        let test1: Vec<Vec<i32>> = manyzip(vec![vec![], vec![1, 2, 3]]).collect();
        let test2: Vec<Vec<i32>> = vec![];

        assert_eq!(test1, test2);
    }

    #[test]
    fn empty_outer() {
        let test1: Vec<Vec<i32>> = manyzip::<Vec<Vec<i32>>>(vec![]).collect();
        let test2: Vec<Vec<i32>> = vec![];

        assert_eq!(test1, test2);
    }
}
