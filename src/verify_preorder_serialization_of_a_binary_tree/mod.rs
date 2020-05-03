// https://leetcode.com/problems/verify-preorder-serialization-of-a-binary-tree/

struct Solution;

impl Solution {
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
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        let pre_order = String::from("9,3,4,#,#,1,#,#,2,#,6,#,#");
        assert_eq!(true, Solution::is_valid_serialization(pre_order));
    }

    #[test]
    fn example2() {
        let pre_order = String::from("1,#");
        assert_eq!(false, Solution::is_valid_serialization(pre_order));
    }

    #[test]
    fn example4() {
        let pre_order = String::from("1,#,#");
        assert_eq!(true, Solution::is_valid_serialization(pre_order));
    }

    #[test]
    fn example3() {
        let pre_order = String::from("9,#,#,1");
        assert_eq!(false, Solution::is_valid_serialization(pre_order));
    }
}
