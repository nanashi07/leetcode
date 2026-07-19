// 1081. Smallest Subsequence of Distinct Characters
// https://leetcode.com/problems/smallest-subsequence-of-distinct-characters/

struct Solution;

impl Solution {
    pub fn smallest_subsequence(s: String) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::smallest_subsequence_of_distinct_characters::Solution;

    #[test]
    fn test_smallest_subsequence_1() {
        let s = "bcabc".to_string();
        assert_eq!("abc".to_string(), Solution::smallest_subsequence(s));
    }

    #[test]
    fn test_smallest_subsequence_2() {
        let s = "cbacdcbc".to_string();
        assert_eq!("acdb".to_string(), Solution::smallest_subsequence(s));
    }
}
