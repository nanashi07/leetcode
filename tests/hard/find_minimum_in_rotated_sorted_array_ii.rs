// 154. Find Minimum in Rotated Sorted Array II
// https://leetcode.com/problems/find-minimum-in-rotated-sorted-array-ii/

struct Solution;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::find_minimum_in_rotated_sorted_array_ii::Solution;

    #[test]
    fn test_find_min_1() {
        let nums = [1, 3, 5].to_vec();
        assert_eq!(1, Solution::find_min(nums));
    }

    #[test]
    fn test_find_min_2() {
        let nums = [2, 2, 2, 0, 1].to_vec();
        assert_eq!(0, Solution::find_min(nums));
    }
}
