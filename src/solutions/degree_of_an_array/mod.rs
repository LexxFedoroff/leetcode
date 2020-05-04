// https://leetcode.com/problems/degree-of-an-array/
use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
        let mut map: HashMap<i32, SubArray> = HashMap::new();

        for (idx, num) in nums.iter().enumerate() {
            let entry = map.entry(*num).or_insert(SubArray {
                min_idx: idx as i32,
                max_idx: idx as i32,
                degree: 0,
            });

            (*entry).max_idx = idx as i32;
            (*entry).degree += 1;
        }

        let mut min_len = nums.len() as i32;
        let mut max_degree = 0;

        for sa in map.values() {
            let degree = sa.degree;
            if degree > max_degree {
                max_degree = degree;
                min_len = sa.max_idx - sa.min_idx + 1;
            } else if degree == max_degree {
                min_len = std::cmp::min(min_len, sa.max_idx - sa.min_idx + 1);
            }
        }

        min_len
    }
}

struct SubArray {
    min_idx: i32,
    max_idx: i32,
    degree: i32,
}

#[cfg(test)]
mod tests;
