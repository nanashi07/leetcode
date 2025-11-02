// 12. Integer to Roman
// https://leetcode.com/problems/integer-to-roman/
struct Solution;

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let map = [
            (1000, "M"),
            (900, "CM"),
            (500, "D"),
            (400, "CD"),
            (100, "C"),
            (90, "XC"),
            (50, "L"),
            (40, "XL"),
            (10, "X"),
            (9, "IX"),
            (5, "V"),
            (4, "IV"),
            (1, "I"),
        ];

        let mut num = num;

        let mut r = String::new();
        for (n, d) in map {
            while num >= n {
                num = num - n;
                r.push_str(d);
            }
        }

        r
    }
}

// Symbol       Value
// I             1
// V             5
// X             10
// L             50
// C             100
// D             500
// M             1000

// I can be placed before V (5) and X (10) to make 4 and 9.
// X can be placed before L (50) and C (100) to make 40 and 90.
// C can be placed before D (500) and M (1000) to make 400 and 900.

#[test]
fn test_int_to_roman() {
    let num = 3;
    let result = Solution::int_to_roman(num);
    assert_eq!("III".to_owned(), result, "num = {}", &num);

    let num = 58;
    let result = Solution::int_to_roman(num);
    assert_eq!("LVIII".to_owned(), result, "num = {}", &num);

    let num = 1994;
    let result = Solution::int_to_roman(num);
    assert_eq!("MCMXCIV".to_owned(), result, "num = {}", &num);
}
