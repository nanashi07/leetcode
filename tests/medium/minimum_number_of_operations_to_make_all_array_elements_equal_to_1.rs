// 2654. Minimum Number of Operations to Make All Array Elements Equal to 1
// https://leetcode.com/problems/minimum-number-of-operations-to-make-all-array-elements-equal-to-1/

struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::minimum_number_of_operations_to_make_all_array_elements_equal_to_1::Solution;

    #[test]
    fn test_min_operations_1() {
        let nums = [2, 6, 3, 4].to_vec();
        assert_eq!(4, Solution::min_operations(nums));
    }

    #[test]
    fn test_min_operations_2() {
        let nums = [2, 10, 6, 14].to_vec();
        assert_eq!(-1, Solution::min_operations(nums));
    }
}
