pub struct Windows<const N: usize, T, U> {
    iter: T,
    buf: [Option<U>; N],
    end_steps: usize,
}

impl<const N: usize, T, U> Windows<N, T, U>
where
    T: Iterator<Item = U>,
    U: Clone,
{
    #[inline]
    pub(super) fn new(iter: T) -> Self {
        assert!(N > 0);
        Self {
            iter,
            buf: init_empty_array(),
            end_steps: 0,
        }
    }

    fn advance(&mut self) -> Option<()> {
        let next = self.iter.next();

        self.rotate();

        if next.is_none() {
            // Ensure we walk N-1 steps more and fill None from the right
            if self.end_steps >= N - 1 {
                return None;
            }
            self.end_steps += 1;
        }

        unsafe {
            // Safety: We have ensured that N > 0 so an array with N > 0 items trivially has a last
            // element.
            *self.buf.last_mut().unwrap_unchecked() = next;
        }

        Some(())
    }

    #[inline]
    fn rotate(&mut self) {
        self.buf.rotate_left(1)
    }
}

impl<const N: usize, T, U> Iterator for Windows<N, T, U>
where
    T: Iterator<Item = U>,
    U: Clone,
{
    type Item = [Option<U>; N];

    fn next(&mut self) -> Option<Self::Item> {
        self.advance()?;
        Some(self.buf.clone())
    }
}

#[cfg(test)]
mod tests {
    use crate::itertools::IterExt;

    #[test]
    fn test_windows1() {
        let text = "abcd";
        let mut iter = text.chars().windows::<1>();
        assert_eq!(iter.next(), Some([Some('a')]));
        assert_eq!(iter.next(), Some([Some('b')]));
        assert_eq!(iter.next(), Some([Some('c')]));
        assert_eq!(iter.next(), Some([Some('d')]));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_windows2() {
        let text = "abcd";
        let mut iter = text.chars().windows::<2>();
        assert_eq!(iter.next(), Some([None, Some('a')]));
        assert_eq!(iter.next(), Some([Some('a'), Some('b')]));
        assert_eq!(iter.next(), Some([Some('b'), Some('c')]));
        assert_eq!(iter.next(), Some([Some('c'), Some('d')]));
        assert_eq!(iter.next(), Some([Some('d'), None]));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_windows3() {
        let text = "abcd";
        let mut iter = text.chars().windows::<3>();
        assert_eq!(iter.next(), Some([None, None, Some('a')]));
        assert_eq!(iter.next(), Some([None, Some('a'), Some('b')]));
        assert_eq!(iter.next(), Some([Some('a'), Some('b'), Some('c')]));
        assert_eq!(iter.next(), Some([Some('b'), Some('c'), Some('d')]));
        assert_eq!(iter.next(), Some([Some('c'), Some('d'), None]));
        assert_eq!(iter.next(), Some([Some('d'), None, None]));
        assert_eq!(iter.next(), None);
    }
}

trait EmptyArrayInit: Sized {
    const NONE: Option<Self>;
}

impl<T> EmptyArrayInit for T {
    const NONE: Option<T> = None;
}

#[inline]
pub fn init_empty_array<T, const N: usize>() -> [Option<T>; N] {
    [EmptyArrayInit::NONE; N]
}
