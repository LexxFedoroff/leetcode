use crate::Solution;

#[test]
fn example1() {
    assert_eq!(
        3,
        Solution::length_of_longest_substring("abcabcbb".to_owned())
    );
}

#[test]
fn example2() {
    assert_eq!(1, Solution::length_of_longest_substring("bbbbb".to_owned()));
}

#[test]
fn example3() {
    assert_eq!(
        3,
        Solution::length_of_longest_substring("pwwkew".to_owned())
    );
}
