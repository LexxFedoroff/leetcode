use crate::Solution;

#[test]
fn example1() {
    assert_eq!(0, Solution::str_str("abc".to_owned(), "".to_owned()));
    assert_eq!(2, Solution::str_str("hello".to_owned(), "ll".to_owned()));
    assert_eq!(-1, Solution::str_str("aaaaa".to_owned(), "bba".to_owned()));
}
