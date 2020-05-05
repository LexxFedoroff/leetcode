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
    let input = ["ab", "b", "bab"];
    let expected = ["bab"];

    let output = Solution::find_all_concatenated_words_in_a_dict(to_input(&input));

    assert_equivalent(expected.iter().cloned(), output.iter().map(|s| s.as_str()));
}

#[test]
fn example4() {
    use std::fs::File;
    use std::io::BufReader;
    use std::path::Path;

    let current_dir = std::env::current_dir().unwrap();
    let input_path = current_dir.join(Path::new(
        "./src/solutions/concatenated_words/tests/data1_input.json",
    ));
    let expected_path = current_dir.join(Path::new(
        "./src/solutions/concatenated_words/tests/data1_expected.json",
    ));

    let input_file = File::open(input_path).unwrap();

    let input: serde_json::Value = serde_json::from_reader(BufReader::new(input_file)).unwrap();
    let input = input.as_array().unwrap();
    let input: Vec<&str> = input.iter().filter_map(|v| v.as_str()).collect();

    let expected_file = File::open(expected_path).unwrap();

    let expected: serde_json::Value =
        serde_json::from_reader(BufReader::new(expected_file)).unwrap();
    let expected = expected.as_array().unwrap();
    let expected: Vec<&str> = expected.iter().filter_map(|v| v.as_str()).collect();

    let output = Solution::find_all_concatenated_words_in_a_dict(to_input(&input));

    assert_equivalent(expected.iter().cloned(), output.iter().map(|s| s.as_str()));
}
