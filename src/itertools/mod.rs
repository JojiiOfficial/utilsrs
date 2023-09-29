pub mod char_subs;
pub mod chunked;
pub mod merge;
mod take_while;
pub mod windows;

pub use take_while::take_while;

use self::windows::Windows;
use crate::itertools::chunked::Chunked;

pub trait IterExt<U: Clone>: Iterator<Item = U> + Sized {
    #[inline]
    fn chunked(self, size: usize) -> Chunked<Self, Self::Item> {
        assert!(size > 0);
        Chunked::new(self, size)
    }

    #[inline]
    fn windows<const N: usize>(self) -> Windows<N, Self, Self::Item> {
        Windows::<N, _, _>::new(self)
    }

    /// Advance iterator by `n` steps, or less if the iterator returns None before n steps have
    /// been reached. Returns the actutal amount of steps the iterator made. This is different from
    /// `n` when there are less than `n` elements left in the iterator.
    fn advance(&mut self, n: usize) -> usize {
        let mut c = 0;

        for _ in 0..n {
            if self.next().is_none() {
                break;
            }
            c += 1;
        }

        c
    }
}

impl<T, U> IterExt<U> for T
where
    T: Iterator<Item = U> + Sized,
    U: Clone,
{
}
