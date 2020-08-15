// https://leetcode.com/problems/reverse-nodes-in-k-group/
mod list;

use crate::Solution;
use list::ListNode;

type Link = Option<Box<ListNode>>;

struct List {
    head: Link,
}

fn attach(head: Link, tail: Link) -> Link {
    if tail == None {
        return head;
    }

    if head == None {
        return tail;
    }

    let mut copy = None;

    {
        let mut copy_ref = &mut copy;
        let mut cur = &head;
        while let Some(inner) = cur {
            *copy_ref = Some(Box::new(ListNode::new(inner.val)));
            copy_ref = &mut copy_ref.as_mut().unwrap().next;
            cur = &inner.next;
        }

        let mut cur = &tail;
        while let Some(inner) = cur {
            *copy_ref = Some(Box::new(ListNode::new(inner.val)));
            copy_ref = &mut copy_ref.as_mut().unwrap().next;
            cur = &inner.next;
        }
    }

    return copy;
}

impl List {
    fn reverse_k_group(&mut self, k: i32) {
        let mut prev = None;
        let mut curr = self.head.take();

        let mut i = 0;

        let next = loop {
            if i >= k {
                break curr;
            }

            if let Some(mut curr_inner) = curr {
                let next = curr_inner.next;
                curr_inner.next = prev;
                prev = Some(curr_inner);
                curr = next;
                i += 1;
            } else {
                break None;
            }
        };

        self.head = attach(prev, next);
    }

    fn reverse(&mut self, k: i32) {
        self.reverse_k_group(k);
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if k == 1 {
            return head;
        }

        let mut list = List { head: head };

        list.reverse(k);

        return list.head;
    }
}

#[cfg(test)]
mod tests;
