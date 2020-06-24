// https://leetcode.com/problems/regular-expression-matching/
use crate::Solution;
use std::collections::HashMap;

type MemoKey = (String, String);

fn is_match_aster(s: &str, p: &str, memo: &mut HashMap<MemoKey, bool>) -> bool {
    let mut s = s;
    let p_ch = p.chars().nth(0);

    if s.is_empty() {
        return is_match_rec(&s, &p[2..], memo);
    }

    loop {
        let ch = s.chars().nth(0);

        if ch == p_ch || p_ch == Some('.') {
            let is_match = is_match_rec(&s, &p[2..], memo);
            if is_match {
                return true;
            }
        } else {
            return is_match_rec(&s, &p[2..], memo);
        }

        if s.is_empty() {
            break;
        }
        s = &s[1..];
    }

    return false;
}

fn is_match_rec(s: &str, p: &str, memo: &mut HashMap<MemoKey, bool>) -> bool {
    if let Some(is_match) = memo.get(&(s.to_owned(), p.to_owned())) {
        return *is_match;
    }

    let ch = s.chars().nth(0);
    let p_ch = p.chars().nth(0);
    let pn_ch = p.chars().nth(1);

    let is_match = match (ch, p_ch, pn_ch) {
        (None, None, _) => true,
        (_, _, Some('*')) => is_match_aster(&s, &p, memo),
        (Some(ch), Some(p_ch), _) => {
            if p_ch == '.' || p_ch == ch {
                is_match_rec(&s[1..], &p[1..], memo)
            } else {
                false
            }
        }
        _ => false,
    };

    memo.entry((s.to_owned(), p.to_owned())).or_insert(is_match);

    return is_match;
}

impl Solution {
    #[allow(dead_code)]
    pub fn is_match(s: String, p: String) -> bool {
        return is_match_rec(&s, &p, &mut HashMap::new());
    }
}

#[cfg(test)]
mod tests;
