// 2996. Smallest Missing Integer Greater Than Sequential Prefix Sum
// https://leetcode.com/problems/smallest-missing-integer-greater-than-sequential-prefix-sum/

struct Solution;

impl Solution {
    pub fn missing_integer(nums: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::smallest_missing_integer_greater_than_sequential_prefix_sum::Solution;

    #[test]
    fn test_missing_integer_1() {
        let nums = [1, 2, 3, 2, 5].to_vec();
        assert_eq!(6, Solution::missing_integer(nums));
    }

    #[test]
    fn test_missing_integer_2() {
        let nums = [3, 4, 5, 1, 12, 14, 13].to_vec();
        assert_eq!(15, Solution::missing_integer(nums));
    }
}
