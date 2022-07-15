// # 7. Reverse Integer
// https://leetcode.com/problems/reverse-integer/
struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut r: i32 = 0;

        let mut x: i32 = x;
        loop {
            if r.checked_mul(10).is_none() {
                return 0;
            }

            r = r * 10 + x % 10;
            x = x / 10;

            if x == 0 {
                break;
            }
        }

        if x < 0 {
            r * -1
        } else {
            r
        }
    }
}

#[test]
fn test_reverse() {
    let x = 123;
    let result = Solution::reverse(x);
    assert_eq!(321, result, "x = {}", x);

    let x = -123;
    let result = Solution::reverse(x);
    assert_eq!(-321, result, "x = {}", x);

    let x = 120;
    let result = Solution::reverse(x);
    assert_eq!(21, result, "x = {}", x);

    let x = 1534236469;
    let result = Solution::reverse(x);
    assert_eq!(0, result, "x = {}", x);
}
