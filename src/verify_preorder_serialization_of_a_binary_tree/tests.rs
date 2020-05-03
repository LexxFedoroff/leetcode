use crate::Solution;

#[test]
fn example1() {
    let pre_order = String::from("9,3,4,#,#,1,#,#,2,#,6,#,#");
    assert_eq!(true, Solution::is_valid_serialization(pre_order));
}

#[test]
fn example2() {
    let pre_order = String::from("1,#");
    assert_eq!(false, Solution::is_valid_serialization(pre_order));
}

#[test]
fn example4() {
    let pre_order = String::from("1,#,#");
    assert_eq!(true, Solution::is_valid_serialization(pre_order));
}

#[test]
fn example3() {
    let pre_order = String::from("9,#,#,1");
    assert_eq!(false, Solution::is_valid_serialization(pre_order));
}
