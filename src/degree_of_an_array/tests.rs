use crate::Solution;

#[test]
fn example1() {
    let input = vec![1, 2, 2, 3, 1];
    let output = 2;
    assert_eq!(output, Solution::find_shortest_sub_array(input));
}

#[test]
fn example2() {
    let input = vec![1, 2, 2, 3, 1, 4, 2];
    let output = 6;
    assert_eq!(output, Solution::find_shortest_sub_array(input));
}

#[test]
fn example3() {
    let input = vec![1, 7, 7, 5, 7, 1];
    let output = 4;
    assert_eq!(output, Solution::find_shortest_sub_array(input));
}
