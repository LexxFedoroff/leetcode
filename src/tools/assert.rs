use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;

#[allow(dead_code)]
pub fn assert_equivalent<T>(expected: impl Iterator<Item = T>, actual: impl Iterator<Item = T>)
where
    T: Hash + Eq + Debug,
{
    let expected_hs = expected.collect::<HashSet<T>>();
    let actual_hs = actual.collect::<HashSet<T>>();
    assert_eq!(expected_hs, actual_hs);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        let iter1 = [1, 2, 3];
        let iter2 = [3, 1, 2];
        super::assert_equivalent(iter1.iter(), iter2.iter());
    }
}
