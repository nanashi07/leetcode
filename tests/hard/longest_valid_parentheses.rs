// 32. Longest Valid Parentheses
// https://leetcode.com/problems/longest-valid-parentheses/
struct Solution;

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::longest_valid_parentheses::Solution;

    #[test]
    fn test_longest_valid_parentheses_1() {
        let s = "(()".to_string();
        assert_eq!(2, Solution::longest_valid_parentheses(s));
    }

    #[test]
    fn test_longest_valid_parentheses_2() {
        let s = ")()())".to_string();
        assert_eq!(4, Solution::longest_valid_parentheses(s));
    }

    #[test]
    fn test_longest_valid_parentheses_3() {
        let s = "".to_string();
        assert_eq!(0, Solution::longest_valid_parentheses(s));
    }
}
