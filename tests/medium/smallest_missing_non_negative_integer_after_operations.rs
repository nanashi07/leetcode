// # 2598. Smallest Missing Non-negative Integer After Operations
// https://leetcode.com/problems/smallest-missing-non-negative-integer-after-operations/

struct Solution;

impl Solution {
    pub fn find_smallest_integer(nums: Vec<i32>, value: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::smallest_missing_non_negative_integer_after_operations::Solution;

    #[test]
    fn test_find_smallest_integer_1() {
        let nums = [1, -10, 7, 13, 6, 8].to_vec();
        let value = 5;
        assert_eq!(4, Solution::find_smallest_integer(nums, value));
    }

    #[test]
    fn test_find_smallest_integer_2() {
        let nums = [1, -10, 7, 13, 6, 8].to_vec();
        let value = 7;
        assert_eq!(2, Solution::find_smallest_integer(nums, value));
    }
}
