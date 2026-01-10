// 712. Minimum ASCII Delete Sum for Two Strings
// https://leetcode.com/problems/minimum-ascii-delete-sum-for-two-strings/

struct Solution;

impl Solution {
    pub fn minimum_delete_sum(s1: String, s2: String) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::minimum_ascii_delete_sum_for_two_strings::Solution;

    #[test]
    fn test_minimum_delete_sum_1() {
        let s1 = "sea".to_string();
        let s2 = "eat".to_string();
        assert_eq!(231, Solution::minimum_delete_sum(s1, s2));
    }

    #[test]
    fn test_minimum_delete_sum_2() {
        let s1 = "delete".to_string();
        let s2 = "leet".to_string();
        assert_eq!(403, Solution::minimum_delete_sum(s1, s2));
    }
}
