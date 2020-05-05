// https://leetcode.com/problems/concatenated-words/
use crate::Solution;
use std::collections::HashSet;

impl Solution {
    #[allow(dead_code)]
    pub fn find_all_concatenated_words_in_a_dict(words: Vec<String>) -> Vec<String> {
        let hs = words.iter().collect::<HashSet<&String>>();

        let mut res = Vec::new();
        for word in words.iter() {
            if Self::is_concatenated_word(&word, &hs) {
                res.push(word.to_owned());
            }
        }

        res
    }

    fn is_concatenated_word(word: &String, hs: &HashSet<&String>) -> bool {
        for i in 1..word.len() {
            let (prefix, suffix) = word.split_at(i);
            let prefix_ok = hs.contains(&prefix.to_owned());
            if !prefix_ok {
                continue;
            }
            let suffix_ok = hs.contains(&suffix.to_owned());
            if prefix_ok && suffix_ok {
                return true;
            }

            let suffix_ok = Self::is_concatenated_word(&suffix.to_owned(), hs);
            if prefix_ok && suffix_ok {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests;
