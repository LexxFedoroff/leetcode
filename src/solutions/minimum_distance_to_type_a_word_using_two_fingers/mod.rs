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
    static ref MAP: HashMap<&'static char, (usize, usize)> = {
        let mut map = HashMap::new();
        for (x, line) in KEYBOARD.iter().enumerate() {
            for (y, c) in line.iter().enumerate() {
                map.insert(c, (x, y));
            }
        }
        map
    };
}

fn distance(c1: &char, c2: &char) -> i32 {
    let c1 = MAP[c1];
    let c2 = MAP[c2];

    (c1.0 as i32 - c2.0 as i32).abs() + (c1.1 as i32 - c2.1 as i32).abs()
}

impl Solution {
    #[allow(dead_code)]
    pub fn minimum_distance(word: String) -> i32 {
        1
    }
}

#[cfg(test)]
mod tests;
