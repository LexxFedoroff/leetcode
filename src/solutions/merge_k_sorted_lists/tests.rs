use super::new_with_array;
use crate::Solution;

#[test]
fn example1() {
    let input = vec![
        new_with_array(&[1, 4, 5]),
        new_with_array(&[1, 3, 4]),
        new_with_array(&[2, 6]),
    ];
    assert_eq!(
        new_with_array(&[1, 1, 2, 3, 4, 4, 5, 6]),
        Solution::merge_k_lists(input)
    );
}
