// # 20. Valid Parentheses
// https://leetcode.com/problems/valid-parentheses/
struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut v: Vec<char> = Vec::new();

        for c in s.chars() {
            match c {
                '(' => {
                    v.push(')');
                }
                '[' => {
                    v.push(']');
                }
                '{' => {
                    v.push('}');
                }
                ')' | ']' | '}' => {
                    if v.is_empty() || v.pop().unwrap() != c {
                        return false;
                    }
                }
                _ => return false,
            }
        }

        v.is_empty()
    }
}

#[test]
fn test_is_valid() {
    let s = "()";
    let result = Solution::is_valid(s.to_owned());
    assert_eq!(true, result, "s = {}", s);

    let s = "()[]{}";
    let result = Solution::is_valid(s.to_owned());
    assert_eq!(true, result, "s = {}", s);

    let s = "(]";
    let result = Solution::is_valid(s.to_owned());
    assert_eq!(false, result, "s = {}", s);
}
