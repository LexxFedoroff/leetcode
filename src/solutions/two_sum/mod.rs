// https://leetcode.com/problems/two-sum/
use crate::Solution;
use std::collections::HashMap;

impl Solution {
    #[allow(dead_code)]
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut nums_map = HashMap::<i32, usize>::new();
        for (idx, num) in nums.iter().enumerate() {
            nums_map.insert(*num, idx);
        }

        for (idx, num) in nums.iter().enumerate() {
            let rest = target - num;
            if let Some(idx2) = nums_map.get(&rest) {
                if idx != *idx2 {
                    return vec![idx as i32, *idx2 as i32];
                }
            }
        }

        Vec::new()
    }
}

#[cfg(test)]
mod tests;
