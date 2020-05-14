use crate::Solution;

#[test]
fn example1() {
    let input = "CAKE".to_owned();
    let output = 3;
    assert_eq!(output, Solution::minimum_distance(input));
}

#[test]
fn example2() {
    let input = "HAPPY".to_owned();
    let output = 6;
    assert_eq!(output, Solution::minimum_distance(input));
}

#[test]
fn example3() {
    let input = "NEW".to_owned();
    let output = 3;
    assert_eq!(output, Solution::minimum_distance(input));
}

#[test]
fn example4() {
    let input = "YEAR".to_owned();
    let output = 7;
    assert_eq!(output, Solution::minimum_distance(input));
}

#[test]
fn example5() {
    let input = "JDX".to_owned();
    let output = 1;
    assert_eq!(output, Solution::minimum_distance(input));
}

#[test]
fn example7() {
    let input = "CAUSX".to_owned();
    let output = 8;
    assert_eq!(output, Solution::minimum_distance(input));
}

#[test]
fn example8() {
    let input = "QIBZR".to_owned();
    let output = 7;
    assert_eq!(output, Solution::minimum_distance(input));
}

#[test]
fn example10() {
    let input = "LSGQE".to_owned();
    let output = 6;
    assert_eq!(output, Solution::minimum_distance(input));
}

// #[test]
fn example6() {
    let input = "OPVUWZLCKTDPSUKGHAXIDWHLZFKNBDZEWHBSURTVCADUGTSDMCLDBTAGFWDPGXZBVARNTDICHCUJLNFBQOBTDWMGILXPSFWVGYBZVFFKQIDTOVFAPVNSQJULMVIERWAOXCKXBRI".to_owned();
    let output = 295;
    assert_eq!(output, Solution::minimum_distance(input));
}

// #[test]
fn example9() {
    let input = "KQHHYFJLNVBETAMCWCENJRNXESNJULCHOULUQBMNANXKOGLJPCFZDIDRTWEZWOMFYNNFHOKQELOUUCPYGJAWOTOAGJDYUJRTENWYPCVPYHRYMIUADIVBAIMQSWMODXIL".to_owned();
    let output = 264;
    assert_eq!(output, Solution::minimum_distance(input));
}
