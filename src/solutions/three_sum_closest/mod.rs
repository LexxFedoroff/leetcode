// https://leetcode.com/problems/3sum-closest/
use crate::Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut result = None;
        let n = nums.len();
        for i in 0..n {
            let i_val = nums[i];
            for j in i + 1..n {
                let j_val = nums[j];
                for k in j + 1..n {
                    let k_val = nums[k];
                    let sum = i_val + j_val + k_val;
                    result = match result {
                        None => Some(sum),
                        Some(min_sum) => {
                            if (target - min_sum).abs() < (target - sum).abs() {
                                Some(min_sum)
                            } else {
                                Some(sum)
                            }
                        }
                    }
                }
            }
        }

        return result.unwrap();
    }
}

#[cfg(test)]
mod tests;
