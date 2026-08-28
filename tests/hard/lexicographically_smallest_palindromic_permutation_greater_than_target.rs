// 3734. Lexicographically Smallest Palindromic Permutation Greater Than Target
// https://leetcode.com/problems/lexicographically-smallest-palindromic-permutation-greater-than-target/

struct Solution;

impl Solution {
    pub fn lex_palindromic_permutation(s: String, target: String) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::lexicographically_smallest_palindromic_permutation_greater_than_target::Solution;

    #[test]
    fn test_lex_palindromic_permutation_1() {
        let s = "baba".to_string();
        let target = "abba".to_string();
        assert_eq!(
            "baab".to_string(),
            Solution::lex_palindromic_permutation(s, target)
        );
    }

    #[test]
    fn test_lex_palindromic_permutation_2() {
        let s = "baba".to_string();
        let target = "bbaa".to_string();
        assert_eq!(
            "".to_string(),
            Solution::lex_palindromic_permutation(s, target)
        );
    }

    #[test]
    fn test_lex_palindromic_permutation_3() {
        let s = "abc".to_string();
        let target = "abb".to_string();
        assert_eq!(
            "".to_string(),
            Solution::lex_palindromic_permutation(s, target)
        );
    }

    #[test]
    fn test_lex_palindromic_permutation_4() {
        let s = "aac".to_string();
        let target = "abb".to_string();
        assert_eq!(
            "aca".to_string(),
            Solution::lex_palindromic_permutation(s, target)
        );
    }
}
