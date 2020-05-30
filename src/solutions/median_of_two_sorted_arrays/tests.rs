use crate::Solution;

#[test]
fn example1() {
    let nums1 = vec![1, 3];
    let nums2 = vec![2];
    assert_eq!(2f64, Solution::find_median_sorted_arrays(nums1, nums2));
}

#[test]
fn example2() {
    let nums1 = vec![1, 3, 5];
    let nums2 = vec![4, 4, 7];
    assert_eq!(4f64, Solution::find_median_sorted_arrays(nums1, nums2));
}

#[test]
fn example3() {
    let nums1 = vec![1, 2];
    let nums2 = vec![3, 4];
    assert_eq!(2.5f64, Solution::find_median_sorted_arrays(nums1, nums2));
}
