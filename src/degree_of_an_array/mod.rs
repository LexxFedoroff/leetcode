/// https://leetcode.com/problems/degree-of-an-array/
use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
        let mut map: HashMap<i32, SubArray> = HashMap::new();

        for (idx, num) in nums.iter().enumerate() {
            let entry = map.entry(*num).or_insert(SubArray {
                min_idx: idx as i32,
                max_idx: idx as i32,
                degree: 0,
            });

            (*entry).max_idx = idx as i32;
            (*entry).degree += 1;
        }

        let mut min_len = nums.len() as i32;
        let mut max_degree = 0;

        for sa in map.values() {
            let degree = sa.degree;
            if degree > max_degree {
                max_degree = degree;
                min_len = sa.max_idx - sa.min_idx + 1;
            } else if degree == max_degree {
                min_len = std::cmp::min(min_len, sa.max_idx - sa.min_idx + 1);
            }
        }

        min_len
    }
}

struct SubArray {
    min_idx: i32,
    max_idx: i32,
    degree: i32,
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        let input = vec![1, 2, 2, 3, 1];
        let output = 2;
        assert_eq!(output, Solution::find_shortest_sub_array(input));
    }

    #[test]
    fn example2() {
        let input = vec![1, 2, 2, 3, 1, 4, 2];
        let output = 6;
        assert_eq!(output, Solution::find_shortest_sub_array(input));
    }

    #[test]
    fn example3() {
        let input = vec![1, 7, 7, 5, 7, 1];
        let output = 4;
        assert_eq!(output, Solution::find_shortest_sub_array(input));
    }
}
