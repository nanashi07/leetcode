// # 3461. Check If Digits Are Equal in String After Operations I
// https://leetcode.com/problems/check-if-digits-are-equal-in-string-after-operations-i/

struct Solution;

impl Solution {
    pub fn has_same_digits(s: String) -> bool {
        todo!()
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
