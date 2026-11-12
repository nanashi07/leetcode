// 3105. Longest Strictly Increasing or Strictly Decreasing Subarray
// https://leetcode.com/problems/longest-strictly-increasing-or-strictly-decreasing-subarray/

struct Solution;

impl Solution {
    pub fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::longest_strictly_increasing_or_strictly_decreasing_subarray::Solution;

    #[test]
    fn test_longest_monotonic_subarray_1() {
        let nums: Vec<i32> = vec![1, 4, 3, 3, 2];
        assert_eq!(2, Solution::longest_monotonic_subarray(nums));
    }

    #[test]
    fn test_longest_monotonic_subarray_2() {
        let nums: Vec<i32> = vec![3, 3, 3, 3];
        assert_eq!(2, Solution::longest_monotonic_subarray(nums));
    }

    #[test]
    fn test_longest_monotonic_subarray_3() {
        let nums: Vec<i32> = vec![3, 2, 1];
        assert_eq!(2, Solution::longest_monotonic_subarray(nums));
    }
}
