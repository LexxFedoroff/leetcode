use crate::Solution;

#[test]
fn example1() {
    let input = "CAKE".to_owned();
    let output = 3;
    assert_eq!(output, Solution::minimum_distance(input));
}

#[test]
fn example2() {
    let input = "HAPPY".to_owned();
    let output = 6;
    assert_eq!(output, Solution::minimum_distance(input));
}

#[test]
fn example3() {
    let input = "NEW".to_owned();
    let output = 3;
    assert_eq!(output, Solution::minimum_distance(input));
}

#[test]
fn example4() {
    let input = "YEAR".to_owned();
    let output = 7;
    assert_eq!(output, Solution::minimum_distance(input));
}

#[test]
fn example5() {
    let input = "JDX".to_owned();
    let output = 1;
    assert_eq!(output, Solution::minimum_distance(input));
}
