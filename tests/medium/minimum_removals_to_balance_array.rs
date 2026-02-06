// 3634. Minimum Removals to Balance Array
// https://leetcode.com/problems/minimum-removals-to-balance-array/

struct Solution;

impl Solution {
    pub fn min_removal(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        if n <= 1 {
            return 0;
        }

        // Group elements by their remainder when divided by k
        let mut remainder_groups: std::collections::HashMap<i32, Vec<i32>> =
            std::collections::HashMap::new();

        for &num in &nums {
            let remainder = num % k;
            remainder_groups
                .entry(remainder)
                .or_insert_with(Vec::new)
                .push(num);
        }

        let threshold = 3 * k;
        let mut max_len = 0;

        // For each remainder group, find the longest subarray where max - min <= 3*k
        for (_, mut group) in remainder_groups {
            if group.is_empty() {
                continue;
            }

            // Sort the group
            group.sort();

            // Apply sliding window on this group
            let mut left = 0;
            for right in 0..group.len() {
                while group[right] - group[left] > threshold {
                    left += 1;
                }
                max_len = max_len.max(right - left + 1);
            }
        }

        // Minimum removals = total elements - max valid subarray length
        (n - max_len) as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::minimum_removals_to_balance_array::Solution;

    #[test]
    fn test_min_removal_1() {
        let nums = [2, 1, 5].to_vec();
        let k = 2;
        assert_eq!(1, Solution::min_removal(nums, k));
    }

    #[test]
    fn test_min_removal_2() {
        let nums = [1, 6, 2, 9].to_vec();
        let k = 3;
        assert_eq!(2, Solution::min_removal(nums, k));
    }

    #[test]
    fn test_min_removal_3() {
        let nums = [4, 6].to_vec();
        let k = 2;
        assert_eq!(0, Solution::min_removal(nums, k));
    }

    #[test]
    fn test_min_removal_4() {
        let nums = [12, 18].to_vec();
        let k = 2;
        assert_eq!(0, Solution::min_removal(nums, k));
    }

    #[test]
    fn test_min_removal_5() {
        let nums = [2, 12].to_vec();
        let k = 2;
        assert_eq!(1, Solution::min_removal(nums, k));
    }
}
