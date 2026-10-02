// 22. Generate Parentheses
// https://leetcode.com/problems/generate-parentheses/
struct Solution;

impl Solution {
    // wrong answer
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::generate_parentheses::Solution;
    use crate::shared::vec2d::to_string_vec;

    #[test]
    fn test_generate_parenthesis_1() {
        let n = 3;
        let output = to_string_vec(["((()))", "(()())", "(())()", "()(())", "()()()"]);
        assert_eq!(output, Solution::generate_parenthesis(n));
    }

    #[test]
    fn test_generate_parenthesis_2() {
        let n = 1;
        let output = to_string_vec(["()"]);
        assert_eq!(output, Solution::generate_parenthesis(n));
    }

    #[test]
    fn test_generate_parenthesis_3() {
        let n = 4;
        let output = to_string_vec([
            "(((())))", "((()()))", "((())())", "((()))()", "(()(()))", "(()()())", "(()())()",
            "(())(())", "(())()()", "()((()))", "()(()())", "()(())()", "()()(())", "()()()()",
        ]);
        assert_eq!(output, Solution::generate_parenthesis(n));
    }
}
