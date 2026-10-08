// 1021. Remove Outermost Parentheses
// https://leetcode.com/problems/remove-outermost-parentheses/

struct Solution;

impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        // One pass over the ASCII bytes, keeping a paren only while its depth is
        // inside a primitive: ')' closes before the check, '(' opens after it,
        // so both parens of an outermost pair sit at depth 0 and are skipped.
        let mut out = String::with_capacity(s.len());
        let mut depth = 0i32;
        for b in s.bytes() {
            if b == b')' {
                depth -= 1;
            }
            if depth > 0 {
                out.push(b as char);
            }
            if b == b'(' {
                depth += 1;
            }
        }
        out
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
