use crate::tools::assert::*;
use crate::Solution;

#[test]
fn example1() {
    let input: Vec<String> = [
        "cat",
        "cats",
        "catsdogcats",
        "dog",
        "dogcatsdog",
        "hippopotamuses",
        "rat",
        "ratcatdogcat",
    ]
    .iter()
    .map(|&s| String::from(s))
    .collect();
    let output = vec!["catsdogcats", "dogcatsdog", "ratcatdogcat"];

    assert_equivalent(
        output.iter().cloned(),
        Solution::find_all_concatenated_words_in_a_dict(input)
            .iter()
            .map(|s| s.as_str()),
    );
}

#[test]
fn exapmle2() {
    unimplemented!();
}
