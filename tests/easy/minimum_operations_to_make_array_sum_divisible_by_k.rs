// 3512. Minimum Operations to Make Array Sum Divisible by K
// https://leetcode.com/problems/minimum-operations-to-make-array-sum-divisible-by-k/

struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::minimum_operations_to_make_array_sum_divisible_by_k::Solution;

    #[test]
    fn test_min_operations_1() {
        let nums = [3, 9, 7].to_vec();
        let k = 5;
        assert_eq!(4, Solution::min_operations(nums, k));
    }

    #[test]
    fn test_min_operations_2() {
        let nums = [4, 1, 3].to_vec();
        let k = 4;
        assert_eq!(0, Solution::min_operations(nums, k));
    }

    #[test]
    fn test_min_operations_3() {
        let nums = [3, 2].to_vec();
        let k = 6;
        assert_eq!(5, Solution::min_operations(nums, k));
    }
}
