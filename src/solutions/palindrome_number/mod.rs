// https://leetcode.com/problems/palindrome-number/
use crate::Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }

        let x_as_array = x_as_array(x);
        let n = x_as_array.len();
        for i in 0..(n / 2) {
            if x_as_array[i] != x_as_array[n - i - 1] {
                return false;
            }
        }

        true
    }
}

fn x_as_array(x: i32) -> Box<[i32]> {
    let mut array = Vec::new();

    let mut x = x;

    loop {
        let rem = x % 10;
        x = x / 10;
        array.insert(0, rem);
        if x == 0 {
            break;
        }
    }

    array.into_boxed_slice()
}

#[cfg(test)]
mod tests;
