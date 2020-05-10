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

struct State {
    left_finger: Option<(i32, i32)>,
    right_finger: Option<(i32, i32)>,
}

impl State {
    fn new() -> State {
        State {
            left_finger: None,
            right_finger: None,
        }
    }
}

fn calc_distance(ch: &char, state: &mut State) -> i32 {
    let ch = MAP[ch];

    let ld = if let Some(ref point) = state.left_finger {
        distance(&ch, point)
    } else {
        if ch.1 <= 2 {
            0
        } else {
            i32::MAX
        }
    };

    let rd = if let Some(ref point) = state.right_finger {
        distance(&ch, point)
    } else {
        if ch.1 >= 3 {
            0
        } else {
            i32::MAX
        }
    };

    if ld <= rd {
        state.left_finger = Some(ch);
        ld
    } else {
        state.right_finger = Some(ch);
        rd
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn minimum_distance(word: String) -> i32 {
        let mut dist = 0;

        let mut state = State::new();

        for c in word.chars() {
            dist += calc_distance(&c, &mut state)
        }

        dist
    }
}

#[cfg(test)]
mod tests;
