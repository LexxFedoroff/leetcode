use crate::Solution;

#[test]
fn example1() {
    assert_eq!(42, Solution::my_atoi("42".to_owned()));
    assert_eq!(42, Solution::my_atoi("    42".to_owned()));
    assert_eq!(42, Solution::my_atoi("42 asd asd ".to_owned()));
    assert_eq!(0, Solution::my_atoi("asds 42".to_owned()));
    assert_eq!(42, Solution::my_atoi("+42asd".to_owned()));
}

#[test]
fn example2() {
    assert_eq!(-42, Solution::my_atoi("-42".to_owned()));
    assert_eq!(-42, Solution::my_atoi("    -42".to_owned()));
    assert_eq!(-42, Solution::my_atoi("-42 asd asd ".to_owned()));
    assert_eq!(0, Solution::my_atoi("    - 42".to_owned()));
    assert_eq!(0, Solution::my_atoi("asds -42".to_owned()));
}

#[test]
fn example3() {
    assert_eq!(-2147483648, Solution::my_atoi("-91283472332".to_owned()));
    assert_eq!(
        2147483647,
        Solution::my_atoi("12312312391283472332".to_owned())
    );
}
