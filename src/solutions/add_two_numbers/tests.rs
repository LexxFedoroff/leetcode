use super::new_with_array;
use crate::Solution;

#[test]
fn example1() {
    let l1 = Some(Box::new(new_with_array(&[2, 4, 3])));
    let l2 = Some(Box::new(new_with_array(&[5, 6, 4])));
    let out = Some(Box::new(new_with_array(&[7, 0, 8])));
    assert_eq!(out, Solution::add_two_numbers(l1, l2));
}

#[test]
fn example2() {
    let l1 = Some(Box::new(new_with_array(&[5])));
    let l2 = Some(Box::new(new_with_array(&[5])));
    let out = Some(Box::new(new_with_array(&[0, 1])));
    assert_eq!(out, Solution::add_two_numbers(l1, l2));
}

#[test]
fn example3() {
    let l1 = Some(Box::new(new_with_array(&[1])));
    let l2 = Some(Box::new(new_with_array(&[9, 9])));
    let out = Some(Box::new(new_with_array(&[0, 0, 1])));
    assert_eq!(out, Solution::add_two_numbers(l1, l2));
}
