use crate::Solution;

#[test]
fn example1() {
    assert_eq!(321, Solution::reverse(123));
}

#[test]
fn example2() {
    assert_eq!(-321, Solution::reverse(-123));
}

#[test]
fn example3() {
    assert_eq!(21, Solution::reverse(120));
}

#[test]
fn example4() {
    assert_eq!(0, Solution::reverse(0));
}

#[test]
fn example5() {
    assert_eq!(0, Solution::reverse(1534236469));
}

#[test]
fn example6() {
    assert_eq!(0, Solution::reverse(-2147483648));
}
