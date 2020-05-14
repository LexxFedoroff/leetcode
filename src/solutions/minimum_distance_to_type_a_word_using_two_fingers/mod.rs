// https://leetcode.com/problems/minimum-distance-to-type-a-word-using-two-fingers/
use crate::Solution;
use std::collections::HashMap;

lazy_static! {
    static ref KEYBOARD: &'static [&'static [char]] = {
        &[
            &['A', 'B', 'C', 'D', 'E', 'F'],
            &['G', 'H', 'I', 'J', 'K', 'L'],
            &['M', 'N', 'O', 'P', 'Q', 'R'],
            &['S', 'T', 'U', 'V', 'W', 'X'],
            &['Y', 'Z'],
        ]
    };
    static ref MAP: HashMap<&'static char, (i32, i32)> = {
        let mut map = HashMap::new();
        for (x, line) in KEYBOARD.iter().enumerate() {
            for (y, c) in line.iter().enumerate() {
                map.insert(c, (x as i32, y as i32));
            }
        }
        map
    };
}

fn distance(c1: &(i32, i32), c2: &(i32, i32)) -> i32 {
    (c1.0 as i32 - c2.0 as i32).abs() + (c1.1 as i32 - c2.1 as i32).abs()
}

fn calc_rec(left: Option<char>, right: Option<char>, tail: &str) -> i32 {
    if tail.len() == 0 {
        return 0;
    }

    let head = tail.chars().next().unwrap();

    let min_tail = std::cmp::min(
        calc_rec(Some(head), right, &tail[1..])
            + match left {
                Some(c) => distance(&MAP[&c], &MAP[&head]),
                None => 0,
            },
        calc_rec(left, Some(head), &tail[1..])
            + match right {
                Some(c) => distance(&MAP[&c], &MAP[&head]),
                None => 0,
            },
    );

    min_tail
}

impl Solution {
    #[allow(dead_code)]
    pub fn minimum_distance(word: String) -> i32 {
        calc_rec(None, None, &word)
    }
}

#[cfg(test)]
mod tests;
