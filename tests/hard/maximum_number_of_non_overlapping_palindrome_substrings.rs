// 2472. Maximum Number of Non-overlapping Palindrome Substrings
// https://leetcode.com/problems/maximum-number-of-non-overlapping-palindrome-substrings/

struct Solution;

impl Solution {
    pub fn max_palindromes(s: String, k: i32) -> i32 {
        todo!()
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
