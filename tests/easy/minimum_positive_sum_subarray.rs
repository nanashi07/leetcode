// 3364. Minimum Positive Sum Subarray
// https://leetcode.com/problems/minimum-positive-sum-subarray/

struct Solution;

impl Solution {
    pub fn minimum_sum_subarray(nums: Vec<i32>, l: i32, r: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::minimum_positive_sum_subarray::Solution;

    #[test]
    fn test_minimum_sum_subarray_1() {
        let nums: Vec<i32> = vec![3, -2, 1, 4];
        let l: i32 = 2;
        let r: i32 = 3;
        assert_eq!(1, Solution::minimum_sum_subarray(nums, l, r));
    }

    #[test]
    fn test_minimum_sum_subarray_2() {
        let nums: Vec<i32> = vec![-2, 2, -3, 1];
        let l: i32 = 2;
        let r: i32 = 3;
        assert_eq!(-1, Solution::minimum_sum_subarray(nums, l, r));
    }

    #[test]
    fn test_minimum_sum_subarray_3() {
        let nums: Vec<i32> = vec![1, 2, 3, 4];
        let l: i32 = 2;
        let r: i32 = 4;
        assert_eq!(3, Solution::minimum_sum_subarray(nums, l, r));
    }
}
