// # 594. Longest Harmonious Subsequence
// https://leetcode.com/problems/longest-harmonious-subsequence/

struct Solution;
impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::longest_harmonious_subsequence::Solution;

    #[test]
    fn test_find_lhs_1() {
        let nums = [1, 3, 2, 2, 5, 2, 3, 7].to_vec();
        assert_eq!(5, Solution::find_lhs(nums));
    }

    #[test]
    fn test_find_lhs_2() {
        let nums = [1, 2, 3, 4].to_vec();
        assert_eq!(2, Solution::find_lhs(nums));
    }

    #[test]
    fn test_find_lhs_3() {
        let nums = [1, 1, 1, 1].to_vec();
        assert_eq!(0, Solution::find_lhs(nums));
    }
}
