// 3510. Minimum Pair Removal to Sort Array II
// https://leetcode.com/problems/minimum-pair-removal-to-sort-array-ii/

struct Solution;

impl Solution {
    pub fn minimum_pair_removal(nums: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::minimum_pair_removal_to_sort_array_ii::Solution;

    #[test]
    fn test_minimum_pair_removal_1() {
        let nums = [5, 2, 3, 1].to_vec();
        assert_eq!(2, Solution::minimum_pair_removal(nums));
    }

    #[test]
    fn test_minimum_pair_removal_2() {
        let nums = [1, 2, 2].to_vec();
        assert_eq!(0, Solution::minimum_pair_removal(nums));
    }
}
