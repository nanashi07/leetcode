// 22. Generate Parentheses
// https://leetcode.com/problems/generate-parentheses/
struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let n = n as usize;
        let mut r: Vec<String> = Vec::new();
        let mut s = String::with_capacity(2 * n);
        Self::backtrack(&mut r, &mut s, 0, 0, n);
        r
    }

    fn backtrack(r: &mut Vec<String>, s: &mut String, open: usize, close: usize, n: usize) {
        if s.len() == 2 * n {
            r.push(s.clone());
            return;
        }
        if open < n {
            s.push('(');
            Self::backtrack(r, s, open + 1, close, n);
            s.pop();
        }
        if close < open {
            s.push(')');
            Self::backtrack(r, s, open, close + 1, n);
            s.pop();
        }
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
