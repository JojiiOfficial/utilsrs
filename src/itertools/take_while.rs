use std::iter::{from_fn, Peekable};

/// Returns an iterator over all elements in `iter` where `pred` returns true. The difference
/// between this and the default take_while is that this function only advances `iter` if the
/// predicate is true. Therefore it needs a peekable iterator instead of a normal one.
#[inline]
pub fn take_while<'a, U, P>(
    iter: &'a mut Peekable<U>,
    mut pred: P,
) -> impl Iterator<Item = U::Item> + 'a
where
    U: Iterator,
    P: FnMut(&U::Item) -> bool + 'a,
{
    from_fn(move || {
        if pred(iter.peek()?) {
            iter.next()
        } else {
            None
        }
    })
}
