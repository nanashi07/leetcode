// 3542. Minimum Operations to Convert All Elements to Zero
// https://leetcode.com/problems/minimum-operations-to-convert-all-elements-to-zero/

struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::minimum_operations_to_convert_all_elements_to_zero::Solution;

    #[test]
    fn test_min_operations_1() {
        let nums = vec![0, 2];
        assert_eq!(1, Solution::min_operations(nums));
    }

    #[test]
    fn test_min_operations_2() {
        let nums = vec![3, 1, 2, 1];
        assert_eq!(3, Solution::min_operations(nums));
    }

    #[test]
    fn test_min_operations_3() {
        let nums = vec![1, 2, 1, 2, 1, 2];
        assert_eq!(4, Solution::min_operations(nums));
    }
}
