// https://leetcode.com/problems/remove-duplicates-from-sorted-array/
use crate::Solution;

fn shift(i: usize, n: usize, nums: &mut Vec<i32>) {
    for j in i..n - 1 {
        nums[j] = nums[j + 1];
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut n = nums.len();
        if n == 0 {
            return 0;
        }

        let mut i = 0;

        while i < n - 1 {
            if nums[i] != nums[i + 1] {
                i += 1;
                continue;
            }

            shift(i, n, nums);
            n -= 1;
        }

        return n as i32;
    }
}

#[cfg(test)]
mod tests;
