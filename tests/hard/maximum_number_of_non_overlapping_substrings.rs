// 1520. Maximum Number of Non-Overlapping Substrings
// https://leetcode.com/problems/maximum-number-of-non-overlapping-substrings/

struct Solution;

impl Solution {
    pub fn max_num_of_substrings(s: String) -> Vec<String> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::maximum_number_of_non_overlapping_substrings::Solution;
    use crate::shared::vec2d::to_string_vec;

    #[test]
    fn test_max_num_of_substrings_1() {
        let s = "adefaddaccc".to_string();
        let output = to_string_vec(["e", "f", "ccc"]);
        assert_eq!(output, Solution::max_num_of_substrings(s));
    }

    #[test]
    fn test_max_num_of_substrings_2() {
        let s = "adefaddaccc".to_string();
        let output = to_string_vec(["e", "f", "ccc"]);
        assert_eq!(output, Solution::max_num_of_substrings(s));
    }
}
