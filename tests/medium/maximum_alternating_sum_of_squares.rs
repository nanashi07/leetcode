// 3727. Maximum Alternating Sum of Squares
// https://leetcode.com/problems/maximum-alternating-sum-of-squares/

struct Solution;

impl Solution {
    pub fn max_alternating_sum(nums: Vec<i32>) -> i64 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::maximum_alternating_sum_of_squares::Solution;

    #[test]
    fn test_max_alternating_sum_1() {
        let nums = [1, 2, 3].to_vec();
        assert_eq!(12, Solution::max_alternating_sum(nums));
    }

    #[test]
    fn test_max_alternating_sum_2() {
        let nums = [1, -1, 2, -2, 3, -3].to_vec();
        assert_eq!(16, Solution::max_alternating_sum(nums));
    }
}
