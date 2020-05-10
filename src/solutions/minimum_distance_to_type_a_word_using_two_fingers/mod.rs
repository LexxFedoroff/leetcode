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

fn distance_over<'a>(iter: impl Iterator<Item = char>) -> i32 {
    let mut dist = 0;
    let mut prev: Option<char> = None;
    for curr in iter {
        prev = match prev {
            None => Some(curr),
            Some(prev) => {
                dist += distance(&MAP[&prev], &MAP[&curr]);
                Some(curr)
            }
        }
    }

    dist
}

fn shuffle(word: String) -> Vec<(String, String)> {
    let left = word.chars().filter(|&s| {
        let point = MAP[&s];
        point.1 <= 2
    });
    let right = word.chars().filter(|&s| {
        let point = MAP[&s];
        point.1 >= 3
    });

    let left = left.collect::<String>();
    let right = right.collect::<String>();

    vec![(left, right)]
}

impl Solution {
    #[allow(dead_code)]
    pub fn minimum_distance(word: String) -> i32 {
        let mut dist: Vec<i32> = Vec::new();

        for (left, right) in shuffle(word) {
            dist.push(distance_over(left.chars()) + distance_over(right.chars()));
        }

        dist.into_iter().min().unwrap()
    }
}

#[cfg(test)]
mod tests;
