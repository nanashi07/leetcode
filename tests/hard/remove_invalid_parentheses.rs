// 301. Remove Invalid Parentheses
// https://leetcode.com/problems/remove-invalid-parentheses/

struct Solution;

impl Solution {
    pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::remove_invalid_parentheses::Solution;
    use crate::shared::vec2d::to_string_vec;

    #[test]
    fn test_remove_invalid_parentheses_1() {
        let s = "()())()".to_string();
        let output = to_string_vec(["(())()", "()()()"]);
        assert_eq!(output, Solution::remove_invalid_parentheses(s));
    }

    #[test]
    fn test_remove_invalid_parentheses_2() {
        let s = "(a)())()".to_string();
        let output = to_string_vec(["(a())()", "(a)()()"]);
        assert_eq!(output, Solution::remove_invalid_parentheses(s));
    }

    #[test]
    fn test_remove_invalid_parentheses_3() {
        let s = ")(".to_string();
        let output = to_string_vec([""]);
        assert_eq!(output, Solution::remove_invalid_parentheses(s));
    }
}
