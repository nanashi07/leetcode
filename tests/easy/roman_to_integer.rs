// 13. Roman to Integer
// https://leetcode.com/problems/roman-to-integer/
use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut n = 0;
        let mut s = &s[..];

        let map2 = HashMap::from([
            ("CM", 900),
            ("CD", 400),
            ("XC", 90),
            ("XL", 40),
            ("IX", 9),
            ("IV", 4),
        ]);

        let map = HashMap::from([
            ("M", 1000),
            ("D", 500),
            ("C", 100),
            ("L", 50),
            ("X", 10),
            ("V", 5),
            ("I", 1),
        ]);

        while !s.is_empty() {
            if s.len() > 1 {
                let k = &s[0..=1];
                if let Some(v) = map2.get(k) {
                    s = &s[2..];
                    n = n + v;
                    continue;
                }
            }
            let k = &s[0..1];
            if let Some(v) = map.get(k) {
                s = &s[1..];
                n = n + v;
            }
        }

        n
    }
}

#[test]
fn test_roman_to_int() {
    let s = "III";
    let result = Solution::roman_to_int(s.to_owned());
    assert_eq!(3, result, "s = {}", &s);

    let s = "LVIII";
    let result = Solution::roman_to_int(s.to_owned());
    assert_eq!(58, result, "s = {}", &s);

    let s = "MCMXCIV";
    let result = Solution::roman_to_int(s.to_owned());
    assert_eq!(1994, result, "s = {}", &s);
}
