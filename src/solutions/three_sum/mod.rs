// https://leetcode.com/problems/3sum/
use crate::Solution;
use std::collections::HashMap;

struct Result {
    unique: HashMap<Vec<i32>, Vec<i32>>,
}

impl Result {
    fn new(capacity: usize) -> Self {
        Result {
            unique: HashMap::with_capacity(capacity),
        }
    }

    fn append(&mut self, a: i32, b: i32, c: i32) {
        let mut vec = vec![a, b, c];
        if !Self::is_zero(&vec) {
            return;
        }
        vec.sort();
        self.unique.entry(vec.clone()).or_insert(vec);
    }

    fn is_zero(vec: &Vec<i32>) -> bool {
        vec.iter().fold(0, |acc, x| acc + x) == 0
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut cache: HashMap<i32, usize> = HashMap::with_capacity(nums.len());

        for (i, i_val) in nums.iter().enumerate() {
            cache.entry(*i_val).or_insert(i);
        }

        let mut res = Result::new(nums.len());
        for i in 0..nums.len() {
            let i_val = nums[i];
            for j in i + 1..nums.len() {
                let j_val = nums[j];
                let k_val = 0 - i_val - j_val;
                if let Some(k) = cache.get(&k_val) {
                    if i != *k && j != *k {
                        res.append(i_val, j_val, k_val);
                    }
                }
            }
        }
        return res.unique.values().cloned().collect();
    }
}

#[cfg(test)]
mod tests;
