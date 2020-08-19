// https://leetcode.com/problems/3sum/
use crate::Solution;
use std::collections::HashSet;

fn is_zero(vec: &Vec<i32>) -> bool {
    vec.iter().fold(0, |acc, x| acc + x) == 0
}

impl Solution {
    #[allow(dead_code)]
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = Vec::default();
        let mut hs: HashSet<Vec<i32>> = HashSet::default();
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                for k in j + 1..nums.len() {
                    let mut vec = vec![nums[i], nums[j], nums[k]];
                    vec.sort();
                    if is_zero(&vec) && !hs.contains(&vec) {
                        hs.insert(vec.clone());
                        res.push(vec);
                    }
                }
            }
        }
        return res;
    }
}

#[cfg(test)]
mod tests;
