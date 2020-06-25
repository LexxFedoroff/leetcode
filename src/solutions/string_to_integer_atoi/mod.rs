// https://leetcode.com/problems/string-to-integer-atoi/
use crate::Solution;

const I32_MAX: i32 = 2147483647i32;
const I32_MIN: i32 = -2147483648i32;

fn is_dig(ch: char) -> Option<i32> {
    match ch {
        '0' => Some(0),
        '1' => Some(1),
        '2' => Some(2),
        '3' => Some(3),
        '4' => Some(4),
        '5' => Some(5),
        '6' => Some(6),
        '7' => Some(7),
        '8' => Some(8),
        '9' => Some(9),
        _ => None,
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn my_atoi(s: String) -> i32 {
        let mut i: Option<i32> = None;
        let mut sign = 1;

        for ch in s.chars() {
            if let Some(dig) = is_dig(ch) {
                i = if let Some(ii) = i {
                    if ii == 0 {
                        Some(dig * sign)
                    } else {
                        ii.checked_mul(10)
                            .and_then(|d| {
                                if sign > 0 {
                                    d.checked_add(dig)
                                } else {
                                    d.checked_sub(dig)
                                }
                            })
                            .or_else(|| {
                                if sign > 0 {
                                    Some(I32_MAX)
                                } else {
                                    Some(I32_MIN)
                                }
                            })
                    }
                } else {
                    Some(dig)
                }
            } else if i == None && ch == '-' {
                sign = -1;
                i = Some(0);
            } else if i == None && ch == '+' {
                sign = 1;
                i = Some(0);
            } else if i == None && ch == ' ' {
                continue;
            } else {
                break;
            }
        }

        return i.or(Some(0)).unwrap();
    }
}

#[cfg(test)]
mod tests;
