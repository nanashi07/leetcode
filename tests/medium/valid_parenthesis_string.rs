// 678. Valid Parenthesis String
// https://leetcode.com/problems/valid-parenthesis-string/

struct Solution;

impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::valid_parenthesis_string::Solution;

    #[test]
    fn test_check_valid_string_1() {
        let s = "()".to_string();
        assert!(Solution::check_valid_string(s));
    }

    #[test]
    fn test_check_valid_string_2() {
        let s = "(*)".to_string();
        assert!(Solution::check_valid_string(s));
    }

    #[test]
    fn test_check_valid_string_3() {
        let s = "(*))".to_string();
        assert!(Solution::check_valid_string(s));
    }

    #[test]
    fn test_check_valid_string_4() {
        let s = "(".to_string();
        assert!(!Solution::check_valid_string(s));
    }
}
