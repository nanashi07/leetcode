// 1930. Unique Length-3 Palindromic Subsequences
// https://leetcode.com/problems/unique-length-3-palindromic-subsequences/

struct Solution;

impl Solution {
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::unique_length_3_palindromic_subsequences::Solution;

    #[test]
    fn test_count_palindromic_subsequence_1() {
        let s = "aabca".to_string();
        assert_eq!(3, Solution::count_palindromic_subsequence(s));
    }

    #[test]
    fn test_count_palindromic_subsequence_2() {
        let s = "adc".to_string();
        assert_eq!(0, Solution::count_palindromic_subsequence(s));
    }

    #[test]
    fn test_count_palindromic_subsequence_3() {
        let s = "bbcbaba".to_string();
        assert_eq!(4, Solution::count_palindromic_subsequence(s));
    }
}
