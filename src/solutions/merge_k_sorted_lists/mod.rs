// https://leetcode.com/problems/merge-k-sorted-lists/
use crate::Solution;
mod list;

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
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut arr = Vec::new();

        let mut heads: Vec<&Option<Box<ListNode>>> = lists.iter().collect();

        loop {
            let mut min: &Option<Box<ListNode>> = &None;
            let mut min_idx = 0;

            for (i, head) in heads.iter().enumerate() {
                min = match (min, head) {
                    (Some(val_min), Some(val_head)) => {
                        if val_min.val < val_head.val {
                            min
                        } else {
                            min_idx = i;
                            head
                        }
                    }
                    (None, Some(_)) => {
                        min_idx = i;
                        head
                    }
                    _ => continue,
                }
            }

            if let Some(min) = min {
                arr.push(min.val);
                heads[min_idx] = &min.next;
            } else {
                break;
            }
        }

        return new_with_array(&arr);
    }
}

#[cfg(test)]
mod tests;
