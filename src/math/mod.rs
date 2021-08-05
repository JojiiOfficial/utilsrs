use std::ops::Sub;

/// Calculates the difference between `a` and `b`. This method never fails.
#[inline]
pub fn diff<T: Sub<Output = T> + PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a - b
    } else {
        b - a
    }
}
