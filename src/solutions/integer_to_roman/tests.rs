use crate::Solution;

#[test]
fn example1() {
    assert_eq!("I", Solution::int_to_roman(1));
    assert_eq!("III", Solution::int_to_roman(3));
    assert_eq!("IV", Solution::int_to_roman(4));
    assert_eq!("IX", Solution::int_to_roman(9));
    assert_eq!("LVIII", Solution::int_to_roman(58));
    assert_eq!("DLVI", Solution::int_to_roman(556));
    assert_eq!("MCMXCIV", Solution::int_to_roman(1994));
}
