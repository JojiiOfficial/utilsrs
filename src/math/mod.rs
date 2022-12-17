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

/// Returns an iterator over all combinations of items in `list`. The permutations are not
/// symmetrical.
/// This means if (a,b) ∈ `list` => (b,a) ∉ `list`.
/// If `reflexive` is true (x,x) ∈ Result, for all x ∈ list
///
/// Example:
///
/// ```rust
/// use permut_combo::*;
/// let list = vec!['a', 'b', 'c'];
/// let res = twos_perm_non_sym(&list, false).map(|i| (*i.0,*i.1)).collect::<Vec<_>>();
/// assert_eq!(res, vec![('a','b'),('a','c'),('b','c')]);
///
/// let res = twos_perm_non_sym(&list, true).map(|i| (*i.0,*i.1)).collect::<Vec<_>>();
/// assert_eq!(res, vec![('a', 'a'), ('a', 'b'), ('a', 'c'), ('b', 'b'), ('b', 'c'), ('c', 'c')]);
/// ```
pub fn twos_perm_non_sym<T>(list: &[T], reflexive: bool) -> impl Iterator<Item = (&T, &T)> {
    let pos_init = if reflexive { 0 } else { 1 };

    // Frist pointer. Always points to the first value in resulting tuple
    let mut fp = 0;
    // Second pointer. Always points to the second value in resulting tuple
    let mut sp = pos_init;

    std::iter::from_fn(move || {
        if list.len().saturating_sub(pos_init) <= fp {
            return None;
        }

        let res = (&list[fp], &list[fp + sp]);

        sp += 1;
        if sp + fp >= list.len() {
            fp += 1;
            sp = pos_init;
        }

        Some(res)
    })
}

/// Returns an iterator over all combinations of items in `list`. The permutations are symmetrical.
/// This means if (a,b) ∈ `list` => (b,a) ∉ `list`.
/// If `reflexive` is true (x,x) ∈ Result, for all x ∈ list
///
/// Example:
///
/// ```rust
/// use permut_combo::*;
/// let list = vec!['a', 'b', 'c'];
/// let res = twos_perm_sym(&list, false).map(|i| (*i.0,*i.1)).collect::<Vec<_>>();
/// assert_eq!(res, vec![('a', 'b'), ('a', 'c'), ('b', 'a'), ('b', 'c'), ('c', 'a'), ('c',
/// 'b')]);
///
/// let res = twos_perm_sym(&list, true).map(|i| (*i.0,*i.1)).collect::<Vec<_>>();
/// assert_eq!(res, vec![('a', 'a'), ('a', 'b'), ('a', 'c'), ('b', 'a'), ('b', 'b'), ('b', 'c'), ('c', 'a'), ('c', 'b'), ('c', 'c')]);
/// ```
pub fn twos_perm_sym<T>(list: &[T], reflexive: bool) -> impl Iterator<Item = (&T, &T)> {
    let pos_init = if reflexive { 0 } else { 1 };

    // Frist pointer. Always points to the first value in resulting tuple
    let mut fp = 0;
    // Second pointer. Always points to the second value in resulting tuple
    let mut sp = pos_init;

    std::iter::from_fn(move || {
        if fp >= list.len() {
            return None;
        }

        let res = (&list[fp], &list[sp]);

        sp += 1;
        if !reflexive && sp == fp {
            sp += 1;
        }
        if sp >= list.len() {
            fp += 1;
            sp = 0;
        }

        Some(res)
    })
}

#[inline]
pub fn pow_mod(base: usize, exponent: u32, modulo: usize) -> usize {
    (base % modulo).pow(exponent) % modulo
}
