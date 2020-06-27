// https://leetcode.com/problems/container-with-most-water/
use crate::Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max = 0;

        for i in 0..height.len() {
            for j in i + 1..height.len() {
                let x = (j - i) as i32;
                let y = std::cmp::min(height[i], height[j]);
                max = std::cmp::max(max, x * y);
            }
        }

        return max;
    }
}

#[cfg(test)]
mod tests;
