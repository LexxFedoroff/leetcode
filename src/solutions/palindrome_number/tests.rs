use crate::Solution;

#[test]
fn example1() {
    assert_eq!(true, Solution::is_palindrome(121));
}
#[test]
fn example2() {
    assert_eq!(false, Solution::is_palindrome(-121));
}
#[test]
fn example3() {
    assert_eq!(false, Solution::is_palindrome(10));
}
#[test]
fn example4() {
    assert_eq!(true, Solution::is_palindrome(0));
}
