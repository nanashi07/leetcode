// 3505. Minimum Operations to Make Elements Within K Subarrays Equal
// https://leetcode.com/problems/minimum-operations-to-make-elements-within-k-subarrays-equal/

struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32, k: i32) -> i64 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::minimum_operations_to_make_elements_within_k_subarrays_equal::Solution;

    #[test]
    fn test_min_operations_1() {
        let nums = [5, -2, 1, 3, 7, 3, 6, 4, -1].to_vec();
        let x = 3;
        let k = 2;
        assert_eq!(8, Solution::min_operations(nums, x, k));
    }

    #[test]
    fn test_min_operations_2() {
        let nums = [9, -2, -2, -2, 1, 5].to_vec();
        let x = 2;
        let k = 2;
        assert_eq!(3, Solution::min_operations(nums, x, k));
    }
}
