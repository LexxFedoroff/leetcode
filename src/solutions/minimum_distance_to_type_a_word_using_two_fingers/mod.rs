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

#[derive(Clone)]
struct State {
    left_finger: Option<(i32, i32)>,
    right_finger: Option<(i32, i32)>,
    distance: i32,
    idx: i32,
    m: (i32, i32, i32),
}

impl State {
    fn new() -> State {
        State {
            left_finger: None,
            right_finger: None,
            distance: 0,
            idx: 0,
            m: (0, 0, 0),
        }
    }

    fn move_left(&self, point: &(i32, i32)) -> State {
        let dist = match self.left_finger {
            Some(p) => distance(&p, point),
            None => 0,
        };
        let idx = self.idx + 1;
        State {
            left_finger: Some(*point),
            right_finger: self.right_finger,
            distance: self.distance + dist,
            idx,
            m: (idx, idx, self.m.2),
        }
    }

    fn move_right(&self, point: &(i32, i32)) -> State {
        let dist = match self.right_finger {
            Some(p) => distance(&p, point),
            None => 0,
        };
        let idx = self.idx + 1;
        State {
            left_finger: self.left_finger,
            right_finger: Some(*point),
            distance: self.distance + dist,
            idx,
            m: (idx, self.m.1, idx),
        }
    }
}

fn calc_rec(tail: &str, state: State, memo: &mut HashMap<(i32, i32, i32), State>) -> State {
    if tail.len() == 0 {
        return state;
    }

    // if let Some(val) = memo.get(&state.m) {
    //     let state_out = val.clone();
    //     return state_out;
    // }

    let head = tail.chars().nth(0).unwrap();

    let left_state = calc_rec(&tail[1..], state.move_left(&MAP[&head]), memo);
    let right_state = calc_rec(&tail[1..], state.move_right(&MAP[&head]), memo);

    let state_out = if left_state.distance < right_state.distance {
        left_state
    } else {
        right_state
    };

    println!("{:?} {:?} {:?}", state.m, state_out.m, state_out.distance);
    // memo.entry(state.m).or_insert(state_out.clone());

    state_out
}

impl Solution {
    #[allow(dead_code)]
    pub fn minimum_distance(word: String) -> i32 {
        let state = calc_rec(&word, State::new(), &mut HashMap::new());
        state.distance
    }
}

#[cfg(test)]
mod tests;
