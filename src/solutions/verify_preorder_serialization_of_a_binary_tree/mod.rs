// https://leetcode.com/problems/verify-preorder-serialization-of-a-binary-tree/
use crate::Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn is_valid_serialization(pre_order: String) -> bool {
        let nodes: Vec<&str> = pre_order.split(',').collect();

        if let Some(idx) = Self::is_tree(&nodes, 0) {
            idx == nodes.len() - 1
        } else {
            false
        }
    }

    fn is_tree(nodes: &[&str], idx: usize) -> Option<usize> {
        if idx >= nodes.len() {
            return None;
        }

        let node = nodes[idx];
        if node == "#" {
            return Some(idx);
        }

        if let Some(idx) = Self::is_tree(nodes, idx + 1) {
            return Self::is_tree(nodes, idx + 1);
        }

        None
    }
}

#[cfg(test)]
mod tests;
