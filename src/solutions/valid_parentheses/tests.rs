use crate::Solution;

#[test]
fn example1() {
    let input = "()".to_owned();
    assert_eq!(true, Solution::is_valid(input));
}

#[test]
fn example2() {
    let input = "()[]{}".to_owned();
    assert_eq!(true, Solution::is_valid(input));
}

#[test]
fn example3() {
    let input = "(]".to_owned();
    assert_eq!(false, Solution::is_valid(input));
}

#[test]
fn example4() {
    let input = "([)]".to_owned();
    assert_eq!(false, Solution::is_valid(input));
}

#[test]
fn example5() {
    let input = "{[]}()".to_owned();
    assert_eq!(true, Solution::is_valid(input));
}
