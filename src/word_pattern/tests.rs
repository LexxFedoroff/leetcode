use crate::Solution;

#[test]
fn example1() {
    let pattern = String::from("abba");
    let string = String::from("dog cat cat dog");
    assert_eq!(true, Solution::word_pattern(pattern, string));
}
#[test]
fn example2() {
    let pattern = String::from("abba");
    let string = String::from("dog cat cat fish");
    assert_eq!(false, Solution::word_pattern(pattern, string));
}
#[test]
fn example3() {
    let pattern = String::from("aaaa");
    let string = String::from("dog cat cat dog");
    assert_eq!(false, Solution::word_pattern(pattern, string));
}
#[test]
fn example4() {
    let pattern = String::from("abba");
    let string = String::from("dog dog dog dog");
    assert_eq!(false, Solution::word_pattern(pattern, string));
}
