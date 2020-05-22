// https://leetcode.com/problems/split-array-largest-sum/
use crate::Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn split_array(nums: Vec<i32>, m: i32) -> i32 {
        Self::split_rec(&nums, m)
    }

    fn split_rec(tail: &[i32], m: i32) -> i32 {
        if m == 1 {
            return tail.iter().sum();
        }

        let n = tail.len();
        let to = 1 + n - m as usize;

        let mut min = None;

        for i in 1..to + 1 as usize {
            let sum1 = tail[0..i].iter().sum();
            let sum2 = Self::split_rec(&tail[i..n], m - 1);
            let max_sum = std::cmp::max(sum1, sum2);
            min = match min {
                Some(m) => Some(std::cmp::min(m, max_sum)),
                None => Some(max_sum),
            };
        }

        min.unwrap()
    }
}

#[cfg(test)]
mod tests;
