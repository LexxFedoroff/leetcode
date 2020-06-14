// https://leetcode.com/problems/longest-palindromic-substring/
use crate::Solution;

fn get_palindrome_odd(s: &Vec<char>, idx: usize) -> &[char] {
    let mut i = 1;
    loop {
        let left = idx.checked_sub(i).and_then(|t| s.get(t));
        let right = s.get(idx + i);
        match (left, right) {
            (Some(l), Some(r)) if l == r => {
                i += 1;
                continue;
            }
            _ => {
                let f = idx - (i - 1);
                let t = idx + (i - 1) + 1;
                return &s[f..t];
            }
        }
    }
}

fn get_palindrome_even(s: &Vec<char>, idx: usize) -> &[char] {
    let mut i = 0;
    loop {
        let left = idx.checked_sub(i).and_then(|t| s.get(t));
        let right = s.get(idx + i + 1);
        match (left, right) {
            (Some(l), Some(r)) if l == r => {
                i += 1;
                continue;
            }
            _ => {
                let f = (idx + 1).checked_sub(i).unwrap_or(0);
                let t = idx + i + 1;
                return &s[f..t];
            }
        }
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn longest_palindrome(s: String) -> String {
        let svec: Vec<char> = s.chars().collect();

        let mut max: &[char] = Default::default();

        for (idx, _) in svec.iter().enumerate() {
            let curr = get_palindrome_odd(&svec, idx);
            max = if max.len() >= curr.len() { max } else { curr };
            let curr = get_palindrome_even(&svec, idx);
            max = if max.len() >= curr.len() { max } else { curr };
        }

        return max.iter().collect();
    }
}

#[cfg(test)]
mod tests;
