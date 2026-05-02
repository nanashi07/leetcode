// 788. Rotated Digits
// https://leetcode.com/problems/rotated-digits/

struct Solution;

impl Solution {
    pub fn rotated_digits(n: i32) -> i32 {
        // Digit DP: count numbers in [1, n] that contain only valid digits
        // (0,1,2,5,6,8,9) and at least one "different" digit (2,5,6,9).
        let digits: Vec<u8> = n.to_string().bytes().map(|b| b - b'0').collect();
        let len = digits.len();
        // dp[pos][tight][has_diff]
        // tight: whether we're still bounded by n's digits
        // has_diff: whether we've seen a 2/5/6/9
        let mut memo = vec![vec![vec![-1i32; 2]; 2]; len];

        fn solve(
            pos: usize,
            tight: bool,
            has_diff: bool,
            digits: &[u8],
            memo: &mut Vec<Vec<Vec<i32>>>,
        ) -> i32 {
            if pos == digits.len() {
                return if has_diff { 1 } else { 0 };
            }
            let cached = memo[pos][tight as usize][has_diff as usize];
            if cached != -1 {
                return cached;
            }
            let limit = if tight { digits[pos] } else { 9 };
            let mut count = 0;
            for d in 0..=limit {
                match d {
                    0 | 1 | 8 => {
                        count += solve(pos + 1, tight && d == limit, has_diff, digits, memo);
                    }
                    2 | 5 | 6 | 9 => {
                        count += solve(pos + 1, tight && d == limit, true, digits, memo);
                    }
                    _ => {} // 3,4,7 invalid
                }
            }
            memo[pos][tight as usize][has_diff as usize] = count;
            count
        }

        solve(0, true, false, &digits, &mut memo)
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::rotated_digits::Solution;

    #[test]
    fn test_rotated_digits_1() {
        let n = 10;
        assert_eq!(4, Solution::rotated_digits(n));
    }

    #[test]
    fn test_rotated_digits_2() {
        let n = 1;
        assert_eq!(0, Solution::rotated_digits(n));
    }

    #[test]
    fn test_rotated_digits_3() {
        let n = 2;
        assert_eq!(1, Solution::rotated_digits(n));
    }
}
