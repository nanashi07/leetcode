// 2472. Maximum Number of Non-overlapping Palindrome Substrings
// https://leetcode.com/problems/maximum-number-of-non-overlapping-palindrome-substrings/

struct Solution;

impl Solution {
    pub fn max_palindromes(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let k = k as usize;
        let n = s.len();
        // Any palindrome can be shrunk (same center) to a palindrome of length k or k+1,
        // so greedy earliest-finish over those window sizes is optimal.
        let pal = |start: usize, len: usize| -> bool {
            start + len <= n && (0..len / 2).all(|d| s[start + d] == s[start + len - 1 - d])
        };
        let mut ans = 0;
        let mut i = 0;
        while i + k <= n {
            if pal(i, k) {
                ans += 1;
                i += k;
            } else if pal(i, k + 1) {
                ans += 1;
                i += k + 1;
            } else {
                i += 1;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::maximum_number_of_non_overlapping_palindrome_substrings::Solution;

    #[test]
    fn test_max_palindromes_1() {
        let s = "abaccdbbd".to_string();
        let k = 3;
        assert_eq!(2, Solution::max_palindromes(s, k));
    }

    #[test]
    fn test_max_palindromes_2() {
        let s = "adbcda".to_string();
        let k = 2;
        assert_eq!(0, Solution::max_palindromes(s, k));
    }
}
