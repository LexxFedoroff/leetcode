// https://leetcode.com/problems/integer-to-roman/
use crate::Solution;

fn det_sym(num: i32) -> (i32, String) {
    let n = num / 1000;
    if n > 0 {
        return (num % 1000, "M".repeat(n as usize));
    }

    let n = num / 100;
    if n > 0 {
        return match n {
            1 | 2 | 3 => (num % 100, "C".repeat(n as usize)),
            4 => (num % 100, "CD".to_owned()),
            5 => (num % 100, "D".to_owned()),
            6 | 7 | 8 => (num % 100, "D".to_owned() + &"C".repeat((n - 5) as usize)),
            9 => (num % 100, "CM".to_owned()),
            _ => panic!(),
        };
    }

    let n = num / 10;
    if n > 0 {
        return match n {
            1 | 2 | 3 => (num % 10, "X".repeat(n as usize)),
            4 => (num % 10, "XL".to_owned()),
            5 => (num % 10, "L".to_owned()),
            6 | 7 | 8 => (num % 10, "L".to_owned() + &"X".repeat((n - 5) as usize)),
            9 => (num % 10, "XC".to_owned()),
            _ => panic!(),
        };
    }

    let n = num;
    return match n {
        1 | 2 | 3 => (0, "I".repeat(n as usize)),
        4 => (0, "IV".to_owned()),
        5 => (0, "V".to_owned()),
        6 | 7 | 8 => (0, "V".to_owned() + &"I".repeat((n - 5) as usize)),
        9 => (0, "IX".to_owned()),
        _ => panic!(),
    };
}

impl Solution {
    #[allow(dead_code)]
    pub fn int_to_roman(num: i32) -> String {
        let mut roman = String::new();
        let mut num = num;
        while num > 0 {
            let (n, sym) = det_sym(num);
            num = n;
            roman.push_str(&sym);
        }
        return roman;
    }
}

#[cfg(test)]
mod tests;
