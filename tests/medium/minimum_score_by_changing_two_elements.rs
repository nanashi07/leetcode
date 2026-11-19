// 2567. Minimum Score by Changing Two Elements
// https://leetcode.com/problems/minimum-score-by-changing-two-elements/

struct Solution;

impl Solution {
    pub fn minimize_sum(nums: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::minimum_score_by_changing_two_elements::Solution;

    #[test]
    fn test_minimize_sum_1() {
        let nums: Vec<i32> = vec![1, 4, 7, 8, 5];
        assert_eq!(3, Solution::minimize_sum(nums));
    }

    #[test]
    fn test_minimize_sum_2() {
        let nums: Vec<i32> = vec![1, 4, 3];
        assert_eq!(0, Solution::minimize_sum(nums));
    }
}
