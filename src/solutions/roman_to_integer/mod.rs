// https://leetcode.com/problems/roman-to-integer/
use crate::Solution;

fn c_to_i(c: char) -> i32 {
    match c {
        'I' => 1,
        'V' => 5,
        'X' => 10,
        'L' => 50,
        'C' => 100,
        'D' => 500,
        'M' => 1000,
        _ => panic!(),
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn roman_to_int(s: String) -> i32 {
        let mut res = 0;
        let mut sub_res = 0;
        let mut prev = 0;
        for c in s.chars() {
            let cur = c_to_i(c);
            if prev == 0 {
                sub_res = cur;
            } else if cur == prev {
                sub_res += cur;
            } else if cur < prev {
                res += sub_res;
                sub_res = cur;
            } else if cur > prev {
                sub_res = cur - sub_res;
            }
            prev = cur;
        }

        res += sub_res;

        res
    }
}

#[cfg(test)]
mod tests;
