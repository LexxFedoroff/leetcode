use crate::Solution;

#[test]
fn example1() {
    let mut input = vec![3, 2, 2, 3];

    assert_eq!(2, Solution::remove_element(&mut input, 3));
    assert_eq!([2, 2], input[0..2]);
}

#[test]
fn example2() {
    let mut input = vec![0, 1, 2, 2, 3, 0, 4, 2];
    assert_eq!(5, Solution::remove_element(&mut input, 2));
    assert_eq!([0, 1, 3, 0, 4], input[0..5]);
}
