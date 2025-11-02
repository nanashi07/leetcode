// 9. Palindrome Number
// https://leetcode.com/problems/palindrome-number/
struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x.is_negative() {
            return false;
        }

        let mut t = x;
        let mut y = 0;
        loop {
            y = y * 10 + t % 10;
            t = t / 10;

            if t == 0 {
                break;
            }
        }

        x == y
    }
}

#[test]
fn test_is_palindrome() {
    let x = 121;
    let result = Solution::is_palindrome(x);
    assert_eq!(true, result, "input = {}", x);

    let x = -121;
    let result = Solution::is_palindrome(x);
    assert_eq!(false, result, "input = {}", x);

    let x = 10;
    let result = Solution::is_palindrome(x);
    assert_eq!(false, result, "input = {}", x);
}
