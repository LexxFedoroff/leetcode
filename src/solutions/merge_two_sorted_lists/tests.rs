use super::new_with_array;
use crate::Solution;

#[test]
fn example1() {
    let list1 = new_with_array(&[1, 2, 4]);
    let list2 = new_with_array(&[1, 3, 4]);
    let ans = new_with_array(&[1, 1, 2, 3, 4, 4]);
    assert_eq!(ans, Solution::merge_two_lists(list1, list2));
}
