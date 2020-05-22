use crate::Solution;

#[test]
fn example1() {
    let nums = vec![7, 2, 5, 10, 8];
    assert_eq!(18, Solution::split_array(nums, 2));
}
