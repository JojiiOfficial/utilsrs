/// An iterator yielding elements chunked
pub struct Chunked<T, U>
where
    T: Iterator<Item = U>,
    U: Clone,
{
    iter: T,
    size: usize,
    buf: Vec<T::Item>,
}

impl<T, U> Chunked<T, U>
where
    T: Iterator<Item = U>,
    U: Clone,
{
    #[inline]
    pub(crate) fn new(iter: T, size: usize) -> Self {
        Self {
            iter,
            size,
            buf: Vec::with_capacity(size),
        }
    }
}

impl<U: Clone, T: Iterator<Item = U>> Iterator for Chunked<T, U> {
    type Item = Vec<T::Item>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let next = self.iter.next();
            let empty = next.is_none();

            if empty && self.buf.is_empty() {
                return None;
            }

            if let Some(next) = next {
                self.buf.push(next);
            }

            if self.buf.len() >= self.size || empty {
                let res = Some(self.buf.clone());
                self.buf.clear();
                return res;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::itertools::IterExt;

    #[test]
    fn chunked_iter() {
        let mut iter = (0..10).chunked(2);
        assert_eq!(iter.next(), Some(vec![0, 1]));
        assert_eq!(iter.next(), Some(vec![2, 3]));
        assert_eq!(iter.next(), Some(vec![4, 5]));
        assert_eq!(iter.next(), Some(vec![6, 7]));
        assert_eq!(iter.next(), Some(vec![8, 9]));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn chunked_iter_empty() {
        let mut iter = std::iter::from_fn::<String, _>(|| None).chunked(1);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn chunked_non_align() {
        let mut iter = (0..11).chunked(2);
        assert_eq!(iter.next(), Some(vec![0, 1]));
        assert_eq!(iter.next(), Some(vec![2, 3]));
        assert_eq!(iter.next(), Some(vec![4, 5]));
        assert_eq!(iter.next(), Some(vec![6, 7]));
        assert_eq!(iter.next(), Some(vec![8, 9]));
        assert_eq!(iter.next(), Some(vec![10]));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn chunked_size_larger() {
        let mut iter = (0..11).chunked(20);
        assert_eq!(iter.next(), Some((0..11).collect::<Vec<_>>()));
        assert_eq!(iter.next(), None);
    }
}
