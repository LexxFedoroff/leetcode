// https://leetcode.com/problems/reverse-integer/
use crate::Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn reverse(x: i32) -> i32 {
        let sign = if x < 0 { -1 } else { 1 };
        let x = match x.checked_mul(sign) {
            Some(d) => d,
            None => return 0,
        };
        let rem = x % 10;
        let mut div = x / 10;
        let mut res = rem * 1;

        while div > 0 {
            let rem = div % 10;
            div = div / 10;
            res = match res.checked_mul(10).and_then(|d| d.checked_add(rem)) {
                Some(d) => d,
                None => return 0,
            }
        }

        res * sign
    }
}

#[cfg(test)]
mod tests;
