// # 2163. Minimum Difference in Sums After Removal of Elements
// https://leetcode.com/problems/minimum-difference-in-sums-after-removal-of-elements/

struct Solution;

impl Solution {
    pub fn minimum_difference(nums: Vec<i32>) -> i64 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::minimum_difference_in_sums_after_removal_of_elements::Solution;

    #[test]
    fn test_minimum_difference_1() {
        let nums = vec![3, 1, 2];
        assert_eq!(-1, Solution::minimum_difference(nums));
    }

    #[test]
    fn test_minimum_difference_2() {
        let nums = vec![7, 9, 5, 8, 1, 3];
        assert_eq!(1, Solution::minimum_difference(nums));
    }
}
