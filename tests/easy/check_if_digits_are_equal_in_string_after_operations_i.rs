// 3461. Check If Digits Are Equal in String After Operations I
// https://leetcode.com/problems/check-if-digits-are-equal-in-string-after-operations-i/

struct Solution;

impl Solution {
    pub fn has_same_digits(s: String) -> bool {
        println!("s: {}", &s);

        let mut s = s;

        while s.len() > 2 {
            let mut sc = String::new();
            let ci = s.as_str();
            for i in 0..ci.len() - 1 {
                let c = (ci[i..i + 1].parse::<i32>().unwrap()
                    + ci[i + 1..i + 2].parse::<i32>().unwrap())
                    % 10;
                sc.push_str(&c.to_string())
            }
            s = sc;
        }

        s.as_str()[0..1] == s.as_str()[1..2]
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::check_if_digits_are_equal_in_string_after_operations_i::Solution;

    #[test]
    fn test_has_same_digits_1() {
        let s = "3902".to_string();
        assert_eq!(true, Solution::has_same_digits(s));
    }

    #[test]
    fn test_has_same_digits_2() {
        let s = "34789".to_string();
        assert_eq!(false, Solution::has_same_digits(s));
    }
}
