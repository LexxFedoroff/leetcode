// https://leetcode.com/problems/regular-expression-matching/
use crate::Solution;

fn split_pattern(p: &str) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();

    for x in p.chars() {
        if x == '*' {
            if let Some(l) = res.last_mut() {
                l.push(x);
            } else {
                panic!();
            }
        } else {
            res.push(x.to_string());
        }
    }

    return res;
}

fn is_match_char(ch: Option<char>, pch: Option<char>) -> bool {
    if pch == Some('.') && ch != None {
        return true;
    }

    return ch == pch;
}

fn try_match(s: &str, p: &str) -> Option<String> {
    match p.len() {
        1 => {
            return if is_match_char(s.chars().nth(0), p.chars().nth(0)) {
                Some(s[1..].to_owned())
            } else {
                None
            }
        }
        2 => {
            let ch = p.chars().nth(0);
            let mut s = s;
            while is_match_char(s.chars().nth(0), ch) {
                s = &s[1..];
            }
            return Some(s.to_owned());
        }
        _ => panic!(),
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn is_match(s: String, p: String) -> bool {
        let mut ms = s;
        for pp in split_pattern(&p).iter() {
            match try_match(&ms, pp) {
                Some(ss) => ms = ss,
                None => return false,
            }
        }

        return ms.is_empty();
    }
}

#[cfg(test)]
mod tests;
