// # 1695. Maximum Erasure Value
// https://leetcode.com/problems/maximum-erasure-value/

struct Solution;

impl Solution {
    pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::maximum_erasure_value::Solution;

    #[test]
    fn test_maximum_unique_subarray_1() {
        let nums = [4, 2, 4, 5, 6].to_vec();
        assert_eq!(17, Solution::maximum_unique_subarray(nums));
    }

    #[test]
    fn test_maximum_unique_subarray_2() {
        let nums = [5, 2, 1, 2, 5, 2, 1, 2, 5].to_vec();
        assert_eq!(8, Solution::maximum_unique_subarray(nums));
    }
}
