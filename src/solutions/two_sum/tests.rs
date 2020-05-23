use crate::Solution;

#[test]
fn example1() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let expected = vec![0, 1];
    assert_eq!(expected, Solution::two_sum(nums, target));
}
