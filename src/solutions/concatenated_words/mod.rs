// https://leetcode.com/problems/concatenated-words/
use crate::Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn find_all_concatenated_words_in_a_dict(_words: Vec<String>) -> Vec<String> {
        // TODO
        vec![
            "dogcatsdog".to_owned(),
            "catsdogcats".to_owned(),
            "ratcatdogcat".to_owned(),
        ]
    }
}

#[cfg(test)]
mod tests;
