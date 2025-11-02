// 32. Longest Valid Parentheses
// https://leetcode.com/problems/longest-valid-parentheses/
struct Solution;

impl Solution {
    // https://leetcode.com/problems/longest-valid-parentheses/discuss/14126/My-O(n)-solution-using-a-stack
    pub fn longest_valid_parentheses(s: String) -> i32 {
        // give a init value
        let mut stk = vec![-1];
        let mut ans = 0;

        for i in 0..s.len() {
            if s.chars().nth(i).unwrap() == '(' {
                stk.push(i as i32);
            } else {
                stk.pop();
                if stk.is_empty() {
                    // push the start position
                    stk.push(i as i32);
                } else {
                    // calculate max length from start position
                    ans = ans.max(i as i32 - stk[stk.len() - 1])
                }
            }
        }

        ans
    }
}

#[test]
fn test_longest_valid_parentheses() {
    let s = "(()";
    let result = Solution::longest_valid_parentheses(s.to_owned());
    assert_eq!(2, result);

    let s = ")()())";
    let result = Solution::longest_valid_parentheses(s.to_owned());
    assert_eq!(4, result);

    let s = "";
    let result = Solution::longest_valid_parentheses(s.to_owned());
    assert_eq!(0, result);

    let s = "()(()";
    let result = Solution::longest_valid_parentheses(s.to_owned());
    assert_eq!(2, result);
}
