// 3381. Maximum Subarray Sum With Length Divisible by K
// https://leetcode.com/problems/maximum-subarray-sum-with-length-divisible-by-k/

struct Solution;

impl Solution {
    pub fn max_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::maximum_subarray_sum_with_length_divisible_by_k::Solution;

    #[test]
    fn test_max_subarray_sum_1() {
        let nums = [1, 2].to_vec();
        let k = 1;
        assert_eq!(3, Solution::max_subarray_sum(nums, k));
    }

    #[test]
    fn test_max_subarray_sum_2() {
        let nums = [-1, -2, -3, -4, -5].to_vec();
        let k = 4;
        assert_eq!(-10, Solution::max_subarray_sum(nums, k));
    }

    #[test]
    fn test_max_subarray_sum_3() {
        let nums = [-5, 1, 2, -3, 4].to_vec();
        let k = 2;
        assert_eq!(4, Solution::max_subarray_sum(nums, k));
    }
}
