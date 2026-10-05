// 856. Score of Parentheses
// https://leetcode.com/problems/score-of-parentheses/

struct Solution;

impl Solution {
    pub fn score_of_parentheses(s: String) -> i32 {
        let mut score: i32 = 0;
        let mut depth: u32 = 0;
        let mut open_prev = false;

        for byte in s.bytes() {
            if byte == b'(' {
                depth += 1;
                open_prev = true;
            } else {
                depth -= 1;
                // A primitive "()" at nesting level `depth` contributes 2^depth.
                if open_prev {
                    score += 1i32 << depth;
                }
                open_prev = false;
            }
        }

        score
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
