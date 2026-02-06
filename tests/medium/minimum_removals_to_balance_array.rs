// 3634. Minimum Removals to Balance Array
// https://leetcode.com/problems/minimum-removals-to-balance-array/

struct Solution;

impl Solution {
    pub fn min_removal(nums: Vec<i32>, k: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::minimum_removals_to_balance_array::Solution;

    #[test]
    fn test_min_removal_1() {
        let nums = [2, 1, 5].to_vec();
        let k = 2;
        assert_eq!(1, Solution::min_removal(nums, k));
    }

    #[test]
    fn test_min_removal_2() {
        let nums = [1, 6, 2, 9].to_vec();
        let k = 3;
        assert_eq!(2, Solution::min_removal(nums, k));
    }

    #[test]
    fn test_min_removal_3() {
        let nums = [4, 6].to_vec();
        let k = 2;
        assert_eq!(0, Solution::min_removal(nums, k));
    }
}
