// https://leetcode.com/problems/remove-element/
use crate::Solution;

fn shift(i: usize, n: usize, nums: &mut Vec<i32>) -> usize {
    for j in i..n - 1 {
        nums[j] = nums[j + 1]
    }
    return n - 1;
}

impl Solution {
    #[allow(dead_code)]
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut n = nums.len();
        let mut i = 0;
        while i < n {
            if nums[i] == val {
                n = shift(i, n, nums);
            } else {
                i += 1;
            }
        }

        return n as i32;
    }
}

#[cfg(test)]
mod tests;
