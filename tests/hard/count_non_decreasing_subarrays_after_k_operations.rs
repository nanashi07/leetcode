// 3420. Count Non-Decreasing Subarrays After K Operations
// https://leetcode.com/problems/count-non-decreasing-subarrays-after-k-operations/

struct Solution;

impl Solution {
    pub fn count_non_decreasing_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::count_non_decreasing_subarrays_after_k_operations::Solution;

    #[test]
    fn test_count_non_decreasing_subarrays_1() {
        let nums: Vec<i32> = vec![6, 3, 1, 2, 4, 4];
        let k: i32 = 7;
        assert_eq!(17, Solution::count_non_decreasing_subarrays(nums, k));
    }

    #[test]
    fn test_count_non_decreasing_subarrays_2() {
        let nums: Vec<i32> = vec![6, 3, 1, 3, 6];
        let k: i32 = 4;
        assert_eq!(12, Solution::count_non_decreasing_subarrays(nums, k));
    }
}
