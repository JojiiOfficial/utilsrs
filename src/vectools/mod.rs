use std::cmp::Ordering;

/// Return true if both slices have the same elments without being stored to be in the same order
#[inline]
pub fn same_elements<T>(v1: &[T], v2: &[T]) -> bool
where
    T: PartialEq,
{
    if v1.len() != v2.len() {
        return false;
    }

    for i in v1 {
        if !v2.contains(&i) {
            return false;
        }
    }

    true
}

/// Return true if [`v1`] is a subset of [`v2`]
#[inline]
pub fn part_of<T>(v1: &[T], v2: &[T]) -> bool
where
    T: PartialEq,
{
    if v1.len() > v2.len() || v1.is_empty() {
        return false;
    }

    for i in v1 {
        if !v2.contains(&i) {
            return false;
        }
    }

    true
}

/// Returns the cutset of both slices as newly allocated vector
#[inline]
pub fn union_elements<'a, T>(v1: &'a [T], v2: &'a [T]) -> Vec<&'a T>
where
    T: PartialEq,
{
    v1.iter().filter(|i| v2.contains(i)).collect::<Vec<_>>()
}

/// Get the relative order of two elements within a vector.
/// Requires that a, b being element of vec.
/// In case there are more elements that would match, the first matching one will be handled.
pub fn get_item_order<T>(vec: &[T], a: &T, b: &T) -> Option<Ordering>
where
    T: PartialEq,
{
    if a == b {
        return Some(Ordering::Equal);
    }

    for i in vec {
        if *i == *a {
            return Some(Ordering::Less);
        }
        if *i == *b {
            return Some(Ordering::Greater);
        }
    }

    None
}

/// Remove duplicates from a vector and return a newly allocated one using a func to compare both
/// items. This doesn't need the source
/// vector to be sorted unlike `.dedup()`. Therefore it's heavier in workload
pub fn remove_dups_by<T, F>(inp: Vec<T>, eq: F) -> Vec<T>
where
    T: PartialEq,
    F: Fn(&T, &T) -> bool,
{
    let mut new: Vec<T> = Vec::new();

    for item in inp {
        if !contains(&new, &item, &eq) {
            new.push(item)
        }
    }

    new
}

/// Remove duplicates from a vector and return a newly allocated one. This doesn't need the source
/// vector to be sorted unlike `.dedup()`. Therefore it's heavier in workload
#[inline]
pub fn remove_dups<T>(inp: Vec<T>) -> Vec<T>
where
    T: PartialEq,
{
    remove_dups_by(inp, |a, b| a == b)
}

#[inline]
fn contains<T, F>(inp: &[T], item: &T, eq: F) -> bool
where
    F: Fn(&T, &T) -> bool,
{
    inp.iter().any(|i| eq(i, item))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_contains() {
        let a = (0..10).collect::<Vec<_>>();

        assert!(contains(&a, &1, |a, b| a == b));
        assert!(contains(&a, &3, |a, b| a == b));
        assert!(!contains(&a, &10, |a, b| a == b));
        assert!(!contains(&a, &100, |a, b| a == b));
    }
}
