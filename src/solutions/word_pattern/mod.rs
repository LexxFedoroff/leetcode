// https://leetcode.com/problems/word-pattern/

use crate::Solution;
use std::collections::HashMap;

impl Solution {
    #[allow(dead_code)]
    pub fn word_pattern(pattern: String, string: String) -> bool {
        let words: Vec<&str> = string.split(' ').collect();
        if pattern.len() != words.len() {
            return false;
        }

        let mut map: HashMap<char, &str> = HashMap::new();
        let mut map2: HashMap<&str, char> = HashMap::new();

        for (idx, ch) in pattern.chars().enumerate() {
            let entry = map.entry(ch).or_insert(words[idx]);
            let entry2 = map2.entry(words[idx]).or_insert(ch);
            if (*entry) != words[idx] || (*entry2) != ch {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests;
