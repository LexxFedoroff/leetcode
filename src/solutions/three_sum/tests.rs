use crate::Solution;

#[test]
fn example1() {
    let empty: Vec<Vec<i32>> = Vec::default();
    assert_eq!(empty, Solution::three_sum(Vec::default()));

    let input = vec![-1, 0, 1, 2, -1, -4];
    let output = vec![vec![-1, 0, 1], vec![-1, -1, 2]];
    assert_eq!(output, Solution::three_sum(input));

    let input = vec![3, 0, -2, -1, 1, 2];
    let output = vec![vec![-2, -1, 3], vec![-2, 0, 2], vec![-1, 0, 1]];
    assert_eq!(output, Solution::three_sum(input));
}
