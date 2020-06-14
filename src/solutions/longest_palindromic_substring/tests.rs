use crate::Solution;

#[test]
fn example1() {
    let input = "babad".to_owned();
    assert_eq!("bab".to_owned(), Solution::longest_palindrome(input));
}

#[test]
fn example2() {
    let input = "cbbd".to_owned();
    assert_eq!("bb".to_owned(), Solution::longest_palindrome(input));
}

#[test]
fn example3() {
    let input = "abc".to_owned();
    assert_eq!("a".to_owned(), Solution::longest_palindrome(input));
}

#[test]
fn example4() {
    let input = "abcba".to_owned();
    assert_eq!("abcba".to_owned(), Solution::longest_palindrome(input));
}

#[test]
fn example5() {
    let input = "abb".to_owned();
    assert_eq!("bb".to_owned(), Solution::longest_palindrome(input));
}

#[test]
fn example6() {
    let input = "mwwfjysbkebpdjyabcfkgprtxpwvhglddhmvaprcvrnuxifcrjpdgnktvmggmguiiquibmtviwjsqwtchkqgxqwljouunurcdtoeygdqmijdympcamawnlzsxucbpqtuwkjfqnzvvvigifyvymfhtppqamlgjozvebygkxawcbwtouaankxsjrteeijpuzbsfsjwxejtfrancoekxgfyangvzjkdskhssdjvkvdskjtiybqgsmpxmghvvicmjxqtxdowkjhmlnfcpbtwvtmjhnzntxyfxyinmqzivxkwigkondghzmbioelmepgfttczskvqfejfiibxjcuyevvpawybcvvxtxycrfbcnpvkzryrqujqaqhoagdmofgdcbhvlwgwmsmhomknbanvntspvvhvccedzzngdywuccxrnzbtchisdwsrfdqpcwknwqvalczznilujdrlevncdsyuhnpmheukottewtkuzhookcsvctsqwwdvfjxifpfsqxpmpwospndozcdbfhselfdltmpujlnhfzjcgnbgprvopxklmlgrlbldzpnkhvhkybpgtzipzotrgzkdrqntnuaqyaplcybqyvidwcfcuxinchretgvfaepmgilbrtxgqoddzyjmmupkjqcypdpfhpkhitfegickfszermqhkwmffdizeoprmnlzbjcwfnqyvmhtdekmfhqwaftlyydirjnojbrieutjhymfpflsfemkqsoewbojwluqdckmzixwxufrdpqnwvwpbavosnvjqxqbosctttxvsbmqpnolfmapywtpfaotzmyjwnd".to_owned();
    assert_eq!("khvhk".to_owned(), Solution::longest_palindrome(input));
}
