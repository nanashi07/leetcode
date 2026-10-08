// 1021. Remove Outermost Parentheses
// https://leetcode.com/problems/remove-outermost-parentheses/

struct Solution;

impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::remove_outermost_parentheses::Solution;

    #[test]
    fn test_remove_outer_parentheses_1() {
        let s = "(()())(())".to_string();
        assert_eq!("()()()".to_string(), Solution::remove_outer_parentheses(s));
    }

    #[test]
    fn test_remove_outer_parentheses_2() {
        let s = "(()())(())(()(()))".to_string();
        assert_eq!(
            "()()()()(())".to_string(),
            Solution::remove_outer_parentheses(s)
        );
    }

    #[test]
    fn test_remove_outer_parentheses_3() {
        let s = "()()".to_string();
        assert_eq!("".to_string(), Solution::remove_outer_parentheses(s));
    }
}
