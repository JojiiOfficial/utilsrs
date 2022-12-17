use std::{collections::HashMap, hash::Hash, marker::PhantomData};

/// Adapter trait to run .merge() on iterators
pub trait MergeExt<R, LI, RI, LF, RF, K>
where
    Self: IntoIterator<Item = LI> + Sized,
{
    fn merge<IR>(self, other: IR, lf: LF, rf: RF) -> Merge<Self, R, K, LI, RI, LF, RF>
    where
        IR: IntoIterator<IntoIter = R>;
}

impl<S: ?Sized, R, LI, RI, LF, RF, K> MergeExt<R, LI, RI, LF, RF, K> for S
where
    Self: IntoIterator<Item = LI, IntoIter = S> + Sized,
    S: Iterator<Item = LI>,
    R: Iterator<Item = RI>,
    K: Hash + Eq,
    LF: Fn(&LI) -> K,
    RF: Fn(&RI) -> K,
{
    #[inline]
    fn merge<IR>(self, other: IR, lf: LF, rf: RF) -> Merge<S, R, K, LI, RI, LF, RF>
    where
        IR: IntoIterator<IntoIter = R>,
    {
        Merge::new(self, other, lf, rf)
    }
}

/// Merges items from iterator using two functions that return a value/key from each item of the
/// iterator. If both iterator return the same items each call this merge is equal to .zip()
pub struct Merge<L, R, K, LT, RT, LF, RF> {
    l_iter: L,
    r_iter: R,
    lf: LF,
    rf: RF,
    buf: HashMap<K, RT>,
    k: PhantomData<K>,
    lt: PhantomData<LT>,
}

impl<L, R, K, LT, RT, LF, RF> Merge<L, R, K, LT, RT, LF, RF>
where
    L: Iterator<Item = LT>,
    R: Iterator<Item = RT>,
    K: Hash + Eq,
    LF: Fn(&LT) -> K,
    RF: Fn(&RT) -> K,
{
    #[inline]
    pub fn new<IL, IR>(l_iter: IL, r_iter: IR, lf: LF, rf: RF) -> Self
    where
        IL: IntoIterator<IntoIter = L>,
        IR: IntoIterator<IntoIter = R>,
    {
        Self {
            l_iter: l_iter.into_iter(),
            r_iter: r_iter.into_iter(),
            lf,
            rf,
            buf: HashMap::new(),
            k: PhantomData,
            lt: PhantomData,
        }
    }

    #[inline]
    fn k_eq(&self, li: &LT, ri: &RT) -> bool {
        (self.lf)(li) == (self.rf)(ri)
    }

    #[inline]
    fn get_buf_for(&mut self, li: &LT) -> Option<RT> {
        self.buf.remove(&(self.lf)(li))
    }

    #[inline]
    fn add_buf(&mut self, ri: RT) {
        let rk = (self.rf)(&ri);
        self.buf.insert(rk, ri);
    }
}

impl<L, R, K, LT, RT, LF, RF> Iterator for Merge<L, R, K, LT, RT, LF, RF>
where
    L: Iterator<Item = LT>,
    R: Iterator<Item = RT>,
    K: Hash + Eq,
    LF: Fn(&LT) -> K,
    RF: Fn(&RT) -> K,
{
    type Item = (LT, RT);

    fn next(&mut self) -> Option<Self::Item> {
        let li = self.l_iter.next()?;

        if let Some(ri) = self.get_buf_for(&li) {
            return Some((li, ri));
        }

        loop {
            let rn = self.r_iter.next()?;

            if self.k_eq(&li, &rn) {
                return Some((li, rn));
            }

            self.add_buf(rn);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_merge() {
        let iter_a = vec![("a", 1), ("b", 2), ("c", 3), ("d", 4)];
        let iter_b = vec![("C", 3), ("B", 2), ("A", 1), ("D", 4), ("Q", 9)];

        let merge = Merge::new(iter_a.iter(), iter_b.iter(), |i| i.1, |i| i.1);

        let cl: Vec<_> = merge.map(|i| (*i.0, *i.1)).collect();
        assert_eq!(
            cl,
            vec![
                (("a", 1), ("A", 1)),
                (("b", 2), ("B", 2)),
                (("c", 3), ("C", 3)),
                (("d", 4), ("D", 4)),
            ]
        );
    }
}
