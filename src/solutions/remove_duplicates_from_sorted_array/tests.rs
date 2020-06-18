use crate::Solution;

#[test]
fn example1() {
    let mut arr = vec![1, 2, 2];
    let n = Solution::remove_duplicates(&mut arr) as usize;
    assert_eq!(&[1, 2], &arr[0..n]);
}

#[test]
fn example4() {
    let mut arr: Vec<i32> = Vec::new();
    let n = Solution::remove_duplicates(&mut arr) as usize;
    assert_eq!(&[] as &[i32; 0], &arr[0..n]);
}

#[test]
fn example3() {
    let mut arr = vec![1, 2, 3];
    let n = Solution::remove_duplicates(&mut arr) as usize;
    assert_eq!(&[1, 2, 3], &arr[0..n]);
}

#[test]
fn example2() {
    let mut arr = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    let n = Solution::remove_duplicates(&mut arr) as usize;
    assert_eq!(&[0, 1, 2, 3, 4], &arr[0..n]);
}
