use super::ListNode;
use crate::Solution;

fn new_with_array(arr: &[i32]) -> Option<Box<ListNode>> {
    let mut next = None;
    for i in arr.iter().rev() {
        next = Some(Box::new(ListNode {
            val: *i,
            next: next,
        }))
    }

    return next;
}

#[test]
fn example1() {
    let input = new_with_array(&[1, 2, 3, 4, 5]);
    // assert_eq!(
    //     new_with_array(&[1, 2, 3, 4, 5]),
    //     Solution::reverse_k_group(input.clone(), 1)
    // );
    assert_eq!(
        new_with_array(&[2, 1, 4, 3, 5]),
        Solution::reverse_k_group(input.clone(), 2)
    );
    // assert_eq!(
    //     new_with_array(&[3, 2, 1, 4, 5]),
    //     Solution::reverse_k_group(input.clone(), 3)
    // );
    // assert_eq!(
    //     new_with_array(&[4, 3, 2, 1, 5]),
    //     Solution::reverse_k_group(input.clone(), 4)
    // );
    // assert_eq!(true, Solution::reverse_k_group(input, 2));
}
