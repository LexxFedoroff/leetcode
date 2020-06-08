// https://leetcode.com/problems/longest-common-prefix/
use crate::Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.len() == 0 {
            return String::default();
        }
        let head = strs[0].chars().collect::<Vec<char>>();
        let tail = strs[1..]
            .iter()
            .map(|s| s.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        let mut prefix = String::default();

        'out: for (idx, ch) in head.iter().enumerate() {
            for svec in tail.iter() {
                if let Some(ch2) = svec.get(idx) {
                    if ch != ch2 {
                        break 'out;
                    }
                } else {
                    break 'out;
                }
            }

            prefix.push(*ch);
        }

        return prefix;
    }
}

#[cfg(test)]
mod tests;
