// https://leetcode.com/problems/add-two-numbers/
mod list;

use crate::Solution;
use list::ListNode;

fn new_with_array(arr: &[i32]) -> ListNode {
    let mut next = None;
    for i in arr.iter().rev() {
        next = Some(Box::new(ListNode {
            val: *i,
            next: next,
        }))
    }

    *next.unwrap()
}

fn reverse(l: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut stack = Vec::new();

    let mut curr = l;

    while let Some(node) = curr {
        stack.push(node.val);
        curr = node.next;
    }

    let list = new_with_array(stack.into_iter().rev().collect::<Vec<i32>>().as_slice());

    return Some(Box::new(list));
}

impl Solution {
    #[allow(dead_code)]
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut curr = None;
        let mut l1 = &l1;
        let mut l2 = &l2;
        let mut extra = 0;
        loop {
            match (l1, l2) {
                (Some(n1), Some(n2)) => {
                    let val = n1.val + n2.val + extra;
                    extra = val / 10;
                    let val = val % 10;

                    let node = Box::new(ListNode {
                        val: val,
                        next: curr,
                    });

                    curr = Some(node);
                    l1 = &n1.next;
                    l2 = &n2.next;
                }
                (Some(n1), None) => {
                    let val = n1.val + extra;
                    extra = val / 10;
                    let val = val % 10;

                    let node = Box::new(ListNode {
                        val: val,
                        next: curr,
                    });

                    curr = Some(node);
                    l1 = &n1.next;
                }
                (None, Some(n2)) => {
                    let val = n2.val + extra;
                    extra = val / 10;
                    let val = val % 10;

                    let node = Box::new(ListNode {
                        val: val,
                        next: curr,
                    });

                    curr = Some(node);
                    l2 = &n2.next;
                }
                (None, None) => {
                    if extra > 0 {
                        let val = extra;
                        extra = val / 10;
                        let val = val % 10;
                        let node = Box::new(ListNode {
                            val: val,
                            next: curr,
                        });
                        curr = Some(node);

                        continue;
                    }

                    break;
                }
            }
        }

        return reverse(curr);
    }
}

#[cfg(test)]
mod tests;
