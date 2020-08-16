// https://leetcode.com/problems/reverse-nodes-in-k-group/
mod list;

use crate::Solution;
use list::ListNode;

fn reverse_rec(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let mut first = head;
    let mut cursor = &mut first;

    for _ in 0..k {
        match cursor {
            None => return first,
            Some(cur_node) => {
                cursor = &mut cur_node.next;
            }
        }
    }

    let mut tail = reverse_rec(cursor.take(), k);
    let mut prev_ref = &mut tail;

    let mut prev;

    for _ in 0..k {
        match first {
            None => unreachable!(),
            Some(mut cur_node) => {
                let next = cur_node.next;
                cur_node.next = prev_ref.take();
                first = next;
                prev = Some(cur_node);
                prev_ref = &mut prev;
            }
        }
    }

    return prev_ref.take();
}

impl Solution {
    #[allow(dead_code)]
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        reverse_rec(head, k)
    }
}

#[cfg(test)]
mod tests;
