// 3942. Minimum Operations to Sort a Permutation
// https://leetcode.com/problems/minimum-operations-to-sort-a-permutation/

struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::minimum_operations_to_sort_a_permutation::Solution;

    #[test]
    fn test_min_operations_1() {
        let nums = [0, 2, 1].to_vec();
        assert_eq!(2, Solution::min_operations(nums.clone()));
    }

    #[test]
    fn test_min_operations_2() {
        let nums = [1, 0, 2].to_vec();
        assert_eq!(2, Solution::min_operations(nums.clone()));
    }

    #[test]
    fn test_min_operations_3() {
        let nums = [2, 0, 1, 3].to_vec();
        assert_eq!(-1, Solution::min_operations(nums.clone()));
    }
}
