// 8. String to Integer (atoi)
// https://leetcode.com/problems/string-to-integer-atoi/
struct Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut start = false;
        let mut negative = 1;
        let mut r: i32 = 0;
        for c in s.chars() {
            let e = c as i32;
            if (e == 45 || e == 43) || e > 47 && e < 58 {
                if e == 45 || e == 43 {
                    if start {
                        break;
                    } else {
                        negative = 44 - e;
                    }
                } else {
                    if r.checked_mul(10).is_none() {
                        return if negative == 1 { i32::MAX } else { i32::MIN };
                    }
                    r = r * 10;
                    if r.checked_add(e - 48).is_none() {
                        return if negative == 1 { i32::MAX } else { i32::MIN };
                    }
                    r = r + e - 48;
                }
                start = true
            } else {
                if !start && e == 32 {
                    // white space leading
                } else {
                    break;
                }
            }
        }

        r * negative
    }
}

#[test]
fn test_my_atoi() {
    let s = "42";
    let result = Solution::my_atoi(s.to_owned());
    assert_eq!(42, result, "input = {:?}", s);

    let s = "   -42";
    let result = Solution::my_atoi(s.to_owned());
    assert_eq!(-42, result, "input = {:?}", s);

    let s = "4193 with words";
    let result = Solution::my_atoi(s.to_owned());
    assert_eq!(4193, result, "input = {:?}", s);

    let s = "words and 987";
    let result = Solution::my_atoi(s.to_owned());
    assert_eq!(0, result, "input = {:?}", s);

    let s = "+1";
    let result = Solution::my_atoi(s.to_owned());
    assert_eq!(1, result, "input = {:?}", s);

    let s = "+-12";
    let result = Solution::my_atoi(s.to_owned());
    assert_eq!(0, result, "input = {:?}", s);

    let s = "2147483648";
    let result = Solution::my_atoi(s.to_owned());
    assert_eq!(2147483647, result, "input = {:?}", s);

    let s = "-5-";
    let result = Solution::my_atoi(s.to_owned());
    assert_eq!(-5, result, "input = {:?}", s);
}
