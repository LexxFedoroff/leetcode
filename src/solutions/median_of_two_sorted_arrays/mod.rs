// https://leetcode.com/problems/median-of-two-sorted-arrays/
use crate::Solution;

fn merge(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut arr: Vec<i32> = Vec::new();
    let mut i1 = 0;
    let mut i2 = 0;
    loop {
        let n1_opt = nums1.get(i1);
        let n2_opt = nums2.get(i2);

        match (n1_opt, n2_opt) {
            (Some(n1), Some(n2)) => {
                if n1 < n2 {
                    arr.push(*n1);
                    i1 += 1;
                } else {
                    arr.push(*n2);
                    i2 += 1;
                }
            }
            (Some(n1), None) => {
                arr.push(*n1);
                i1 += 1;
            }
            (None, Some(n2)) => {
                arr.push(*n2);
                i2 += 1;
            }
            (None, None) => break,
        }
    }

    arr
}

impl Solution {
    #[allow(dead_code)]
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let arr = merge(nums1, nums2);

        let n = arr.len();

        let odd = n % 2 == 1;

        if odd {
            arr[n / 2] as f64
        } else {
            (arr[n / 2 - 1] as f64 + arr[n / 2] as f64) / 2f64
        }
    }
}

#[cfg(test)]
mod tests;
