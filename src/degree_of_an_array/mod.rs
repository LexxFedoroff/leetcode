/// https://leetcode.com/problems/degree-of-an-array/
///

struct Solution;

impl Solution {
    pub fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
        1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        let input = vec![1, 2, 3];
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
