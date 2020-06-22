// https://leetcode.com/problems/zigzag-conversion/
use crate::Solution;

fn r0(s: &Vec<char>, n: i32) -> String {
    let mut i = 0;

    let mut res = String::new();

    loop {
        let idx = i * (n * 2 - 2) as usize;
        if let Some(ch) = s.get(idx) {
            res.push(*ch);
            i += 1;
        } else {
            break;
        }
    }

    return res;
}

fn ri(s: &Vec<char>, ni: i32, n: i32) -> String {
    let mut i = 0;

    let mut res = String::new();

    loop {
        let idx1 = i * (n * 2 - 2) - ni;
        if idx1 >= 0 {
            if let Some(ch) = s.get(idx1 as usize) {
                res.push(*ch);
            } else {
                break;
            }
        }

        let idx2 = i * (n * 2 - 2) + ni;
        if let Some(ch) = s.get(idx2 as usize) {
            res.push(*ch);
        } else {
            break;
        }

        i += 1;
    }

    return res;
}

fn rn(s: &Vec<char>, n: i32) -> String {
    let mut i = 0;

    let mut res = String::new();

    loop {
        let idx = (i * (n * 2 - 2) + n - 1) as usize;
        if let Some(ch) = s.get(idx) {
            res.push(*ch);
            i += 1;
        } else {
            break;
        }
    }

    return res;
}

impl Solution {
    #[allow(dead_code)]
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows <= 1 {
            return s;
        }

        let s: Vec<char> = s.chars().collect();

        let mut res = String::new();

        res.push_str(&r0(&s, num_rows));

        for i in 1..num_rows - 1 {
            res.push_str(&ri(&s, i, num_rows));
        }

        res.push_str(&rn(&s, num_rows));

        return res;
    }
}

#[cfg(test)]
mod tests;
