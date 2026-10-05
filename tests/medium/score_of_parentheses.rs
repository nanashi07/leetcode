// 856. Score of Parentheses
// https://leetcode.com/problems/score-of-parentheses/

struct Solution;

impl Solution {
    pub fn score_of_parentheses(s: String) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::score_of_parentheses::Solution;

    #[test]
    fn test_score_of_parentheses_1() {
        let s = "()".to_string();
        assert_eq!(1, Solution::score_of_parentheses(s));
    }

    #[test]
    fn test_score_of_parentheses_2() {
        let s = "(())".to_string();
        assert_eq!(2, Solution::score_of_parentheses(s));
    }

    #[test]
    fn test_score_of_parentheses_3() {
        let s = "()()".to_string();
        assert_eq!(2, Solution::score_of_parentheses(s));
    }
}
