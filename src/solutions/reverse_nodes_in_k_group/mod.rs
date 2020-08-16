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
        let mut head = self.head.take();
        // let mut cursor = &mut head;
        let mut prev = None;
        // let mut last = &mut cursor;

        let mut i = 0;

        while let Some(mut cur_node) = head {
            // if i >= k {
            //     last.as_mut().unwrap().next = Some(cur_node);
            //     break;
            // }

            let next = cur_node.next.take();
            cur_node.next = prev;
            prev = Some(cur_node);
            head = next;
            i += 1;
        }

        // let mut head = self.head.take();
        // let mut next = head.as_mut().unwrap().next.take();
        // head.as_mut().unwrap().next = next.as_mut().unwrap().next.take();
        // next.as_mut().unwrap().next = head;

        self.head = prev;
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
