use crate::Solution;

#[test]
fn example1() {
    assert_eq!(3, Solution::roman_to_int("III".to_owned()));
}
#[test]
fn example2() {
    assert_eq!(4, Solution::roman_to_int("IV".to_owned()));
}
#[test]
fn example3() {
    assert_eq!(9, Solution::roman_to_int("IX".to_owned()));
}
#[test]
fn example4() {
    assert_eq!(58, Solution::roman_to_int("LVIII".to_owned()));
}
#[test]
fn example5() {
    assert_eq!(1994, Solution::roman_to_int("MCMXCIV".to_owned()));
}
#[test]
fn example6() {
    assert_eq!(12, Solution::roman_to_int("XII".to_owned()));
}
#[test]
fn example7() {
    assert_eq!(78, Solution::roman_to_int("LXXVIII".to_owned()));
}
