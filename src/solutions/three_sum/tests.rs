use crate::Solution;

#[test]
fn example1() {
    let input = vec![-1, 0, 1, 2, -1, -4];
    let output = vec![vec![-1, 0, 1], vec![-1, -1, 2]];
    assert_eq!(output, Solution::three_sum(input));
}
