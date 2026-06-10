// 3691. Maximum Total Subarray Value II
// https://leetcode.com/problems/maximum-total-subarray-value-ii/

struct Solution;

impl Solution {
    pub fn max_total_value(nums: Vec<i32>, k: i32) -> i64 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::maximum_total_subarray_value_ii::Solution;

    #[test]
    fn test_max_total_value_1() {
        let nums = [1, 3, 2].to_vec();
        let k = 2;
        assert_eq!(4, Solution::max_total_value(nums, k));
    }

    #[test]
    fn test_max_total_value_2() {
        let nums = [4, 2, 5, 1].to_vec();
        let k = 3;
        assert_eq!(12, Solution::max_total_value(nums, k));
    }
}
