// 3518. Smallest Palindromic Rearrangement II
// https://leetcode.com/problems/smallest-palindromic-rearrangement-ii/

struct Solution;

impl Solution {
    pub fn smallest_palindrome(s: String, k: i32) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::smallest_palindromic_rearrangement_ii::Solution;

    #[test]
    fn test_smallest_palindrome_1() {
        let s = "abba".to_string();
        let k = 2;
        assert_eq!("baab".to_string(), Solution::smallest_palindrome(s, k));
    }

    #[test]
    fn test_smallest_palindrome_2() {
        let s = "aa".to_string();
        let k = 2;
        assert_eq!("".to_string(), Solution::smallest_palindrome(s, k));
    }

    #[test]
    fn test_smallest_palindrome_3() {
        let s = "bacab".to_string();
        let k = 1;
        assert_eq!("abcba".to_string(), Solution::smallest_palindrome(s, k));
    }
}
