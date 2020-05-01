// https://leetcode.com/problems/word-pattern/

struct Solution;

use std::collections::HashMap;

impl Solution {
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
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        let pattern = String::from("abba");
        let string = String::from("dog cat cat dog");
        assert_eq!(true, Solution::word_pattern(pattern, string));
    }
    #[test]
    fn example2() {
        let pattern = String::from("abba");
        let string = String::from("dog cat cat fish");
        assert_eq!(false, Solution::word_pattern(pattern, string));
    }
    #[test]
    fn example3() {
        let pattern = String::from("aaaa");
        let string = String::from("dog cat cat dog");
        assert_eq!(false, Solution::word_pattern(pattern, string));
    }
    #[test]
    fn example4() {
        let pattern = String::from("abba");
        let string = String::from("dog dog dog dog");
        assert_eq!(false, Solution::word_pattern(pattern, string));
    }
}
