use crate::tools::assert::*;
use crate::Solution;

fn to_input(arr: &[&str]) -> Vec<String> {
    arr.iter().map(|&s| String::from(s)).collect()
}

#[test]
fn example1() {
    let input = [
        "cat",
        "cats",
        "catsdogcats",
        "dog",
        "dogcatsdog",
        "hippopotamuses",
        "rat",
        "ratcatdogcat",
    ];
    let expected = ["catsdogcats", "dogcatsdog", "ratcatdogcat"];
    let output = Solution::find_all_concatenated_words_in_a_dict(to_input(&input));

    assert_equivalent(expected.iter().cloned(), output.iter().map(|s| s.as_str()));
}

#[test]
fn example2() {
    let input = ["ab", "bc", "ca", "a", "abbcbc", "abcbb", "caab"];
    let expected = ["abbcbc", "caab"];

    let output = Solution::find_all_concatenated_words_in_a_dict(to_input(&input));

    assert_equivalent(expected.iter().cloned(), output.iter().map(|s| s.as_str()));
}

#[test]
fn example3() {
    let input = ["a", "b", "ba"];
    let expected = ["ba"];

    let output = Solution::find_all_concatenated_words_in_a_dict(to_input(&input));

    assert_equivalent(expected.iter().cloned(), output.iter().map(|s| s.as_str()));
}
