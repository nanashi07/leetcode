// 2916. Subarrays Distinct Element Sum of Squares II
// https://leetcode.com/problems/subarrays-distinct-element-sum-of-squares-ii/

struct Solution;

impl Solution {
    pub fn sum_counts(nums: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::subarrays_distinct_element_sum_of_squares_ii::Solution;

    #[test]
    fn test_sum_counts_1() {
        let nums = [1, 2, 1].to_vec();
        assert_eq!(15, Solution::sum_counts(nums));
    }

    #[test]
    fn test_sum_counts_2() {
        let nums = [2, 2].to_vec();
        assert_eq!(3, Solution::sum_counts(nums));
    }
}
