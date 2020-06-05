// https://leetcode.com/problems/longest-substring-without-repeating-characters/
use crate::Solution;
use std::collections::HashSet;

fn max_substr(idx: usize, f_idx: &mut usize, str_vec: &Vec<char>, hs: &mut HashSet<char>) -> usize {
    hs.insert(str_vec[idx]);
    *f_idx = std::cmp::max(*f_idx, idx + 1);
    while *f_idx < str_vec.len() && !hs.contains(&str_vec[*f_idx]) {
        hs.insert(str_vec[*f_idx]);
        *f_idx += 1;
    }

    return *f_idx - idx;
}

impl Solution {
    #[allow(dead_code)]
    pub fn length_of_longest_substring(s: String) -> i32 {
        let str_vec = s.chars().collect::<Vec<char>>();

        let mut max = 0;

        let mut idx = 0;
        let n = str_vec.len();

        let mut f_idx = idx + 1;

        let mut hs = HashSet::new();

        loop {
            if idx >= n - max {
                break;
            }

            let cur_max = max_substr(idx, &mut f_idx, &str_vec, &mut hs);

            max = std::cmp::max(cur_max, max);

            hs.remove(&str_vec[idx]);
            idx += 1;
        }

        return max as i32;
    }
}

#[cfg(test)]
mod tests;
