// 1658. Minimum Operations to Reduce X to Zero
// https://leetcode.com/problems/minimum-operations-to-reduce-x-to-zero/

struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::minimum_operations_to_reduce_x_to_zero::Solution;

    #[test]
    fn test_min_operations_1() {
        let nums = [1, 1, 4, 2, 3].to_vec();
        let x = 5;
        assert_eq!(2, Solution::min_operations(nums, x));
    }

    #[test]
    fn test_min_operations_2() {
        let nums = [5, 6, 7, 8, 9].to_vec();
        let x = 4;
        assert_eq!(-1, Solution::min_operations(nums, x));
    }

    #[test]
    fn test_min_operations_3() {
        let nums = [3, 2, 20, 1, 1, 3].to_vec();
        let x = 10;
        assert_eq!(5, Solution::min_operations(nums, x));
    }
}
