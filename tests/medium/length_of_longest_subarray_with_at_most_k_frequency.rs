// 2958. Length of Longest Subarray With at Most K Frequency
// https://leetcode.com/problems/length-of-longest-subarray-with-at-most-k-frequency/

struct Solution;

impl Solution {
    pub fn max_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::length_of_longest_subarray_with_at_most_k_frequency::Solution;

    #[test]
    fn test_max_subarray_length_1() {
        let nums = [1, 2, 3, 1, 2, 3, 1, 2].to_vec();
        let k = 2;
        assert_eq!(6, Solution::max_subarray_length(nums, k));
    }

    #[test]
    fn test_max_subarray_length_2() {
        let nums = [1, 2, 1, 2, 1, 2, 1, 2].to_vec();
        let k = 1;
        assert_eq!(2, Solution::max_subarray_length(nums, k));
    }

    #[test]
    fn test_max_subarray_length_3() {
        let nums = [5, 5, 5, 5, 5, 5, 5].to_vec();
        let k = 4;
        assert_eq!(4, Solution::max_subarray_length(nums, k));
    }
}
