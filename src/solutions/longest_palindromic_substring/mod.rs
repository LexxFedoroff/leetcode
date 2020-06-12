// https://leetcode.com/problems/longest-palindromic-substring/
use crate::Solution;

fn is_polidrom(substr: &Vec<char>) -> bool {
    let n = substr.len();
    for i in 0..(n / 2) {
        if substr[i] != substr[n - i - 1] {
            return false;
        }
    }

    return true;
}

impl Solution {
    #[allow(dead_code)]
    pub fn longest_palindrome(s: String) -> String {
        let n = s.len();

        for x in (0..n + 1).rev() {
            for i in 0..(n - x + 1) {
                if is_polidrom(&s[i..i + x].chars().collect()) {
                    return s[i..i + x].to_owned();
                }
            }
        }

        return String::default();
    }
}

#[cfg(test)]
mod tests;
