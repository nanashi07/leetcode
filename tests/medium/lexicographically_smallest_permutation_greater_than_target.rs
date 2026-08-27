// 3720. Lexicographically Smallest Permutation Greater Than Target
// https://leetcode.com/problems/lexicographically-smallest-permutation-greater-than-target/

struct Solution;

impl Solution {
    pub fn lex_greater_permutation(s: String, target: String) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::lexicographically_smallest_permutation_greater_than_target::Solution;

    #[test]
    fn test_lex_greater_permutation_1() {
        let s = "abc".to_string();
        let target = "bba".to_string();
        assert_eq!(
            "bca".to_string(),
            Solution::lex_greater_permutation(s, target)
        );
    }

    #[test]
    fn test_lex_greater_permutation_2() {
        let s = "leet".to_string();
        let target = "code".to_string();
        assert_eq!(
            "eelt".to_string(),
            Solution::lex_greater_permutation(s, target)
        );
    }

    #[test]
    fn test_lex_greater_permutation_3() {
        let s = "baba".to_string();
        let target = "bbaa".to_string();
        assert_eq!("".to_string(), Solution::lex_greater_permutation(s, target));
    }
}
