use crate::Solution;

#[test]
fn example1() {
    assert_eq!(
        "PAHNAPLSIIGYIR",
        Solution::convert("PAYPALISHIRING".to_owned(), 3)
    );
}

#[test]
fn example2() {
    assert_eq!(
        "PINALSIGYAHRPI",
        Solution::convert("PAYPALISHIRING".to_owned(), 4)
    );
}
