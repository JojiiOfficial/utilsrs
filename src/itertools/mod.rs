use crate::itertools::chunked::Chunked;

pub mod chunked;

pub trait IterExt<U: Clone>: Iterator<Item = U> + Sized {
    #[inline]
    fn chunked(self, size: usize) -> Chunked<Self, Self::Item> {
        assert!(size > 0);
        Chunked::new(self, size)
    }
}

impl<T, U> IterExt<U> for T
where
    T: Iterator<Item = U> + Sized,
    U: Clone,
{
}
