use crate::Solution;

#[test]
fn example1() {
    let input: Vec<String> = ["flower", "flow", "flight"]
        .iter()
        .map(|s| (*s).to_owned())
        .collect();
    assert_eq!("fl".to_owned(), Solution::longest_common_prefix(input));
}

#[test]
fn example2() {
    let input: Vec<String> = ["dog", "racecar", "car"]
        .iter()
        .map(|s| (*s).to_owned())
        .collect();
    assert_eq!("".to_owned(), Solution::longest_common_prefix(input));
}
