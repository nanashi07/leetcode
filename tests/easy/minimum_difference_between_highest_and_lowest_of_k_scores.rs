// 1984. Minimum Difference Between Highest and Lowest of K Scores
// https://leetcode.com/problems/minimum-difference-between-highest-and-lowest-of-k-scores/

struct Solution;

impl Solution {
    pub fn minimum_difference(nums: Vec<i32>, k: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::minimum_difference_between_highest_and_lowest_of_k_scores::Solution;

    #[test]
    fn test_minimum_difference_1() {
        let nums = [90].to_vec();
        let k = 1;
        assert_eq!(0, Solution::minimum_difference(nums, k));
    }

    #[test]
    fn test_minimum_difference_2() {
        let nums = [9, 4, 1, 7].to_vec();
        let k = 2;
        assert_eq!(2, Solution::minimum_difference(nums, k));
    }
}
