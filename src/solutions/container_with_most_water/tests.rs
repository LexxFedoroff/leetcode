use crate::Solution;

#[test]
fn example1() {
    let input = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
    assert_eq!(49, Solution::max_area(input));
}
