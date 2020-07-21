// https://leetcode.com/problems/implement-strstr/
use crate::Solution;

fn is_match(haystack: &[char], needle: &[char]) -> bool {
    for i in 0..needle.len() {
        match (haystack.get(i), needle.get(i)) {
            (Some(a), Some(b)) => {
                if a != b {
                    return false;
                }
            }
            _ => return false,
        }
    }

    return true;
}

impl Solution {
    #[allow(dead_code)]
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.is_empty() {
            return 0;
        }

        let needle = needle.chars().collect::<Vec<char>>();
        let haystack = haystack.chars().collect::<Vec<char>>();

        for (i, _) in haystack.iter().enumerate() {
            if is_match(&haystack[i..], &needle) {
                return i as i32;
            }
        }

        return -1;
    }
}

#[cfg(test)]
mod tests;
