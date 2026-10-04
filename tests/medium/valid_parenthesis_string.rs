// 678. Valid Parenthesis String
// https://leetcode.com/problems/valid-parenthesis-string/

struct Solution;

impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        let (mut min_open, mut max_open) = (0_usize, 0_usize);

        for ch in s.bytes() {
            match ch {
                b'(' => {
                    min_open += 1;
                    max_open += 1;
                }
                b')' => {
                    if max_open == 0 {
                        return false;
                    }
                    min_open = min_open.saturating_sub(1);
                    max_open -= 1;
                }
                b'*' => {
                    min_open = min_open.saturating_sub(1);
                    max_open += 1;
                }
                _ => unreachable!("input contains only parentheses and asterisks"),
            }
        }

        min_open == 0
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
