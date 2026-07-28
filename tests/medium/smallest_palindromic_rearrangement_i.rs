// 3517. Smallest Palindromic Rearrangement I
// https://leetcode.com/problems/smallest-palindromic-rearrangement-i/

struct Solution;

impl Solution {
    pub fn smallest_palindrome(s: String) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::smallest_palindromic_rearrangement_i::Solution;

    #[test]
    fn test_smallest_palindrome_1() {
        let s = "z".to_string();
        assert_eq!("z".to_string(), Solution::smallest_palindrome(s));
    }

    #[test]
    fn test_smallest_palindrome_2() {
        let s = "babab".to_string();
        assert_eq!("abbba".to_string(), Solution::smallest_palindrome(s));
    }

    #[test]
    fn test_smallest_palindrome_3() {
        let s = "daccad".to_string();
        assert_eq!("acddca".to_string(), Solution::smallest_palindrome(s));
    }
}
