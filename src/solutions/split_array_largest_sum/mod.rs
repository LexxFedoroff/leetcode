// https://leetcode.com/problems/split-array-largest-sum/
use crate::Solution;
use std::collections::HashMap;

impl Solution {
    #[allow(dead_code)]
    pub fn split_array(nums: Vec<i32>, m: i32) -> i32 {
        Self::split_rec(&nums, m, &mut HashMap::new())
    }

    fn split_rec(tail: &[i32], m: i32, memo: &mut HashMap<(usize, i32), i32>) -> i32 {
        let n = tail.len();

        if let Some(min) = memo.get(&(n, m)) {
            return *min;
        }

        if m == 1 {
            return tail.iter().sum();
        }

        let to = 1 + n - m as usize;

        let mut min = None;

        for i in 1..to + 1 as usize {
            let sum1 = tail[0..i].iter().sum();
            let sum2 = Self::split_rec(&tail[i..n], m - 1, memo);
            let max_sum = std::cmp::max(sum1, sum2);
            min = match min {
                Some(m) => Some(std::cmp::min(m, max_sum)),
                None => Some(max_sum),
            };
        }

        let min = min.unwrap();
        memo.entry((n, m)).or_insert(min);
        min
    }
}

#[cfg(test)]
mod tests;
