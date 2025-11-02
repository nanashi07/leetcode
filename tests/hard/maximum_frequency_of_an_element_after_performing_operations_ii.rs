// 3347. Maximum Frequency of an Element After Performing Operations II
// https://leetcode.com/problems/maximum-frequency-of-an-element-after-performing-operations-ii/

struct Solution;

impl Solution {
    pub fn max_frequency(nums: Vec<i32>, k: i32, num_operations: i32) -> i32 {
        let mut sorted = nums.clone();
        sorted.sort_unstable();

        let mut max_freq = 0;

        // Helper function to count elements in range [left, right]
        let count_in_range = |left: i32, right: i32| -> i32 {
            let left_idx = sorted.partition_point(|&x| x < left);
            let right_idx = sorted.partition_point(|&x| x <= right);
            (right_idx - left_idx) as i32
        };

        // Helper function to count elements exactly equal to target
        let count_equal = |target: i32| -> i32 {
            let left_idx = sorted.partition_point(|&x| x < target);
            let right_idx = sorted.partition_point(|&x| x <= target);
            (right_idx - left_idx) as i32
        };

        // Collect all candidate target values
        let mut candidates = std::collections::HashSet::new();

        // Add all original values
        for &num in &sorted {
            candidates.insert(num);
        }

        // Add boundary values: for each element, consider target-k and target+k
        for &num in &sorted {
            candidates.insert(num - k);
            candidates.insert(num + k);
        }

        // Try each candidate target value
        for &target in &candidates {
            // Count elements already equal to target
            let equal_count = count_equal(target);

            // Count elements in range [target-k, target+k]
            let in_range_count = count_in_range(target - k, target + k);

            // Elements that can be changed to target (in range but not equal)
            let changeable = in_range_count - equal_count;

            // Maximum frequency for this target
            let freq = equal_count + changeable.min(num_operations);
            max_freq = max_freq.max(freq);
        }

        max_freq
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::maximum_frequency_of_an_element_after_performing_operations_ii::Solution;

    #[test]
    fn test_max_frequency_1() {
        let nums = [1, 4, 5].to_vec();
        let k = 1;
        let num_operations = 2;
        assert_eq!(2, Solution::max_frequency(nums, k, num_operations));
    }

    #[test]
    fn test_max_frequency_2() {
        let nums = [5, 11, 20, 20].to_vec();
        let k = 5;
        let num_operations = 1;
        assert_eq!(2, Solution::max_frequency(nums, k, num_operations));
    }
}
