// 3090. Maximum Length Substring With Two Occurrences
// https://leetcode.com/problems/maximum-length-substring-with-two-occurrences/

struct Solution;

impl Solution {
    pub fn maximum_length_substring(s: String) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::maximum_length_substring_with_two_occurrences::Solution;

    #[test]
    fn test_maximum_length_substring_1() {
        let s = "bcbbbcba".to_string();
        assert_eq!(4, Solution::maximum_length_substring(s));
    }

    #[test]
    fn test_maximum_length_substring_2() {
        let s = "aaaa".to_string();
        assert_eq!(2, Solution::maximum_length_substring(s));
    }
}
