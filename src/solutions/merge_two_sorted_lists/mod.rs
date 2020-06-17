// https://leetcode.com/problems/merge-two-sorted-lists/
mod list;

use crate::Solution;
use list::ListNode;

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

impl Solution {
    #[allow(dead_code)]
    pub fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut arr = Vec::new();

        let mut i1 = &l1;
        let mut i2 = &l2;

        loop {
            match (i1, i2) {
                (Some(val1), Some(val2)) => {
                    if val1.val < val2.val {
                        arr.push(val1.val);
                        i1 = &val1.next;
                    } else {
                        arr.push(val2.val);
                        i2 = &val2.next;
                    }
                }
                (None, Some(val2)) => {
                    arr.push(val2.val);
                    i2 = &val2.next;
                }
                (Some(val1), None) => {
                    arr.push(val1.val);
                    i1 = &val1.next;
                }
                _ => break,
            }
        }

        return new_with_array(&arr);
    }
}

#[cfg(test)]
mod tests;
