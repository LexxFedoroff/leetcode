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
    distance: i32,
}

impl State {
    fn new() -> State {
        State {
            left_finger: None,
            right_finger: None,
            distance: 0,
        }
    }

    fn move_left(&self, point: &(i32, i32)) -> State {
        let dist = match self.left_finger {
            Some(p) => distance(&p, point),
            None => 0,
        };
        State {
            left_finger: Some(*point),
            right_finger: self.right_finger,
            distance: self.distance + dist,
        }
    }

    fn move_right(&self, point: &(i32, i32)) -> State {
        let dist = match self.right_finger {
            Some(p) => distance(&p, point),
            None => 0,
        };
        State {
            left_finger: self.left_finger,
            right_finger: Some(*point),
            distance: self.distance + dist,
        }
    }
}

fn calc_rec(tail: &str, state: State) -> State {
    if tail.len() == 0 {
        return state;
    }

    let head = tail.chars().nth(0).unwrap();

    let left_state = calc_rec(&tail[1..], state.move_left(&MAP[&head]));
    let right_state = calc_rec(&tail[1..], state.move_right(&MAP[&head]));

    if left_state.distance < right_state.distance {
        left_state
    } else {
        right_state
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn minimum_distance(word: String) -> i32 {
        let state = calc_rec(&word, State::new());
        state.distance
    }
}

#[cfg(test)]
mod tests;
