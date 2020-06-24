use crate::Solution;

#[test]
fn example1() {
    assert_eq!(false, Solution::is_match("aa".to_owned(), "a".to_owned()));
}

#[test]
fn example2() {
    assert_eq!(true, Solution::is_match("aa".to_owned(), "a*".to_owned()));
}

#[test]
fn example3() {
    assert_eq!(true, Solution::is_match("ab".to_owned(), ".*".to_owned()));
}

#[test]
fn example4() {
    assert_eq!(
        true,
        Solution::is_match("aabb".to_owned(), "a*b*".to_owned())
    );
}

#[test]
fn example5() {
    assert_eq!(
        true,
        Solution::is_match("aab".to_owned(), "c*a*b".to_owned())
    );
}

#[test]
fn example6() {
    assert_eq!(
        false,
        Solution::is_match("mississippi".to_owned(), "mis*is*p*.".to_owned())
    );
}

#[test]
fn example7() {
    assert_eq!(true, Solution::is_match("aaa".to_owned(), "a*a".to_owned()));
}
