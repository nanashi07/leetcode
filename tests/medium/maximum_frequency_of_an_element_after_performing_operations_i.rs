// 3346. Maximum Frequency of an Element After Performing Operations I
// https://leetcode.com/problems/maximum-frequency-of-an-element-after-performing-operations-i/

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn max_frequency(nums: Vec<i32>, k: i32, num_operations: i32) -> i32 {
        let mut sorted_nums = nums.clone();
        sorted_nums.sort_unstable();

        // Count original frequencies
        let mut freq_map: HashMap<i32, i32> = HashMap::new();
        for &num in &nums {
            *freq_map.entry(num).or_insert(0) += 1;
        }

        let mut max_freq = 0;

        // Try each unique value as a potential target
        let mut candidates = std::collections::HashSet::new();
        for &num in &nums {
            candidates.insert(num);
        }

        for &target in &candidates {
            let range_start = target - k;
            let range_end = target + k;

            // Count how many elements are in range [target-k, target+k]
            let in_range = Self::count_in_range(&sorted_nums, range_start, range_end);

            // Elements already at target value
            let already_at_target = *freq_map.get(&target).unwrap_or(&0);

            // Elements that need to be changed
            let need_change = in_range - already_at_target;

            // We can change at most num_operations elements
            let can_change = need_change.min(num_operations);

            let total_freq = already_at_target + can_change;
            max_freq = max_freq.max(total_freq);
        }

        // Also consider values that no element currently has, but elements can reach
        // For efficiency, check key positions: midpoints between elements
        for i in 0..sorted_nums.len() {
            // Check position slightly to the left
            if i > 0 {
                let target = sorted_nums[i] - k;
                let range_start = target - k;
                let range_end = target + k;
                let in_range = Self::count_in_range(&sorted_nums, range_start, range_end);
                let already_at_target = *freq_map.get(&target).unwrap_or(&0);
                let need_change = in_range - already_at_target;
                let can_change = need_change.min(num_operations);
                let total_freq = already_at_target + can_change;
                max_freq = max_freq.max(total_freq);
            }

            // Check position slightly to the right
            let target = sorted_nums[i] + k;
            let range_start = target - k;
            let range_end = target + k;
            let in_range = Self::count_in_range(&sorted_nums, range_start, range_end);
            let already_at_target = *freq_map.get(&target).unwrap_or(&0);
            let need_change = in_range - already_at_target;
            let can_change = need_change.min(num_operations);
            let total_freq = already_at_target + can_change;
            max_freq = max_freq.max(total_freq);
        }

        max_freq
    }

    fn count_in_range(sorted_nums: &[i32], start: i32, end: i32) -> i32 {
        // Binary search for the range [start, end]
        let left = sorted_nums.partition_point(|&x| x < start);
        let right = sorted_nums.partition_point(|&x| x <= end);
        (right - left) as i32
    }

    pub fn _max_frequency(nums: Vec<i32>, k: i32, num_operations: i32) -> i32 {
        println!(
            "nums: {:?}, k: {k}, num_operations: {num_operations}",
            &nums
        );

        let min = *nums.iter().min().unwrap();
        let max = *nums.iter().max().unwrap();
        let mut duplicated = 0;

        for n in min..=max {
            println!("=========={n}============");
            let mut op = num_operations;
            let mut dup = 0;
            for i in 0..nums.len() {
                if nums[i] - k <= n && nums[i] + k >= n {
                    if nums[i] == n {
                        dup += 1;
                        println!("[{}] dup + 1", &nums[i]);
                    } else {
                        if op > 0 {
                            op -= 1;
                            dup += 1;
                            println!("[{}] op + 1, dup + 1", &nums[i]);
                        }
                    }
                }
            }
            duplicated = duplicated.max(dup);
        }

        duplicated
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::maximum_frequency_of_an_element_after_performing_operations_i::Solution;

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

    #[test]
    fn test_max_frequency_3() {
        let nums = [2, 49].to_vec();
        let k = 97;
        let num_operations = 0;
        assert_eq!(1, Solution::max_frequency(nums, k, num_operations));
    }
}
