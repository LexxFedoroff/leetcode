// https://leetcode.com/problems/valid-parentheses/
use crate::Solution;

fn is_open(c: char) -> bool {
    match c {
        '(' | '[' | '{' => true,
        _ => false,
    }
}

fn is_closed(c: char) -> bool {
    match c {
        ')' | ']' | '}' => true,
        _ => false,
    }
}

fn is_pair(open: char, closed: char) -> bool {
    match open {
        '(' => closed == ')',
        '[' => closed == ']',
        '{' => closed == '}',
        _ => false,
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn is_valid(s: String) -> bool {
        if s.is_empty() {
            return true;
        }

        let mut stack = Vec::new();

        for ch in s.chars() {
            if is_open(ch) {
                stack.push(ch);
                continue;
            } else if is_closed(ch) {
                if let Some(last) = stack.pop() {
                    if !is_pair(last, ch) {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }

        return stack.is_empty();
    }
}

#[cfg(test)]
mod tests;
