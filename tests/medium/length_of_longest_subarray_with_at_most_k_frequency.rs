// 2958. Length of Longest Subarray With at Most K Frequency
// https://leetcode.com/problems/length-of-longest-subarray-with-at-most-k-frequency/

struct Solution;

impl Solution {
    pub fn max_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::HashMap;
        let mut counts = HashMap::new();
        let (mut left, mut max_len) = (0, 0);
        for right in 0..nums.len() {
            *counts.entry(nums[right]).or_insert(0) += 1;
            while counts[&nums[right]] > k {
                *counts.get_mut(&nums[left]).unwrap() -= 1;
                left += 1;
            }
            max_len = max_len.max(right - left + 1);
        }
        max_len as i32
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
