// 2913. Subarrays Distinct Element Sum of Squares I
// https://leetcode.com/problems/subarrays-distinct-element-sum-of-squares-i/

use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn sum_counts(nums: Vec<i32>) -> i32 {
        let mut total = 0;
        for i in 0..nums.len() {
            let mut seen = HashSet::new();
            for &n in nums[i..].iter() {
                seen.insert(n);
                let distinct = seen.len() as i32;
                total += distinct * distinct;
            }
        }
        total
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::subarrays_distinct_element_sum_of_squares_i::Solution;

    #[test]
    fn test_sum_counts_1() {
        let nums = [1, 2, 1].to_vec();
        assert_eq!(15, Solution::sum_counts(nums));
    }

    #[test]
    fn test_sum_counts_2() {
        let nums = [1, 1].to_vec();
        assert_eq!(3, Solution::sum_counts(nums));
    }
}
