// 1877. Minimize Maximum Pair Sum in Array
// https://leetcode.com/problems/minimize-maximum-pair-sum-in-array/

struct Solution;

impl Solution {
    pub fn min_pair_sum(nums: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::minimize_maximum_pair_sum_in_array::Solution;

    #[test]
    fn test_min_pair_sum_1() {
        let nums = [3, 5, 2, 3].to_vec();
        assert_eq!(7, Solution::min_pair_sum(nums));
    }

    #[test]
    fn test_min_pair_sum_2() {
        let nums = [3, 5, 4, 2, 4, 6].to_vec();
        assert_eq!(8, Solution::min_pair_sum(nums));
    }
}
