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

fn calc_rec(left: Option<usize>, right: Option<usize>, head: usize, word: &str) -> i32 {
    if head == word.len() {
        return 0;
    }

    let head_char = word.chars().nth(head).unwrap();
    let left_char = left.and_then(|l| word.chars().nth(l));
    let right_char = right.and_then(|l| word.chars().nth(l));

    let min = std::cmp::min(
        calc_rec(Some(head), right, head + 1, word)
            + match left_char {
                Some(c) => distance(&MAP[&c], &MAP[&head_char]),
                None => 0,
            },
        calc_rec(left, Some(head), head + 1, word)
            + match right_char {
                Some(c) => distance(&MAP[&c], &MAP[&head_char]),
                None => 0,
            },
    );

    min
}

impl Solution {
    #[allow(dead_code)]
    pub fn minimum_distance(word: String) -> i32 {
        calc_rec(None, None, 0, &word)
    }
}

#[cfg(test)]
mod tests;
