// 3634. Minimum Removals to Balance Array
// https://leetcode.com/problems/minimum-removals-to-balance-array/

struct Solution;

impl Solution {
    pub fn min_removal(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        if n <= 1 {
            return 0;
        }

        // Sort the array
        let mut sorted_nums = nums.clone();
        sorted_nums.sort();

        // Find the longest subarray where max - min <= k
        let mut max_len = 0;
        let mut left = 0;

        for right in 0..n {
            // While the difference exceeds k, shrink the window from left
            while sorted_nums[right] - sorted_nums[left] > k {
                left += 1;
            }
            // Update the maximum length
            max_len = max_len.max(right - left + 1);
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
}
