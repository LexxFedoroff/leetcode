// https://leetcode.com/problems/3sum/
use crate::Solution;
use std::collections::HashSet;

struct Result {
    array: Vec<Vec<i32>>,
    unique: HashSet<Vec<i32>>,
}

impl Result {
    fn new() -> Self {
        Result {
            array: Vec::default(),
            unique: HashSet::default(),
        }
    }

    fn append(&mut self, a: i32, b: i32, c: i32) {
        let mut vec = vec![a, b, c];
        if !Self::is_zero(&vec) {
            return;
        }
        vec.sort();
        if !self.unique.contains(&vec) {
            self.unique.insert(vec.clone());
            self.array.push(vec);
        }
    }

    fn is_zero(vec: &Vec<i32>) -> bool {
        vec.iter().fold(0, |acc, x| acc + x) == 0
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = Result::new();
        for i in 0..nums.len() {
            let i_val = nums[i];
            for j in i + 1..nums.len() {
                let j_val = nums[j];
                for k in j + 1..nums.len() {
                    let k_val = nums[k];
                    res.append(i_val, j_val, k_val);
                }
            }
        }
        return res.array;
    }
}

#[cfg(test)]
mod tests;
