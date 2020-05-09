use crate::Solution;

#[test]
fn example1() {
    let input = "CAKE".to_owned();
    let output = 3;
    assert_eq!(output, Solution::minimum_distance(input));
}
