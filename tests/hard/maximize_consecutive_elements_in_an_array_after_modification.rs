// 3041. Maximize Consecutive Elements in an Array After Modification
// https://leetcode.com/problems/maximize-consecutive-elements-in-an-array-after-modification/

use std::collections::HashMap;

struct Solution;

impl Solution {
    /// Every chosen element may be increased by at most 1, and the chosen elements must end up as a
    /// run of consecutive values where each value is used once (the samples rule out keeping two
    /// equal values). After sorting `nums`, `dp[v]` holds the longest chain of
    /// consecutive final values ending at `v` buildable from the elements seen so far; an element
    /// `x` extends a chain ending at `x - 1` (kept as `x`) or one ending at `x` (raised to `x + 1`).
    pub fn max_selected_elements(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();

        let mut dp: HashMap<i32, i32> = HashMap::with_capacity(nums.len() * 2);
        let mut ans = 0;

        for x in nums {
            // Read both transitions before writing so an element is never used twice.
            let keep = dp.get(&(x - 1)).copied().unwrap_or(0) + 1;
            let raise = dp.get(&x).copied().unwrap_or(0) + 1;

            let entry = dp.entry(x).or_default();
            *entry = (*entry).max(keep);
            let entry = dp.entry(x + 1).or_default();
            *entry = (*entry).max(raise);

            ans = ans.max(keep).max(raise);
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::maximize_consecutive_elements_in_an_array_after_modification::Solution;

    #[test]
    fn test_max_selected_elements_1() {
        let nums: Vec<i32> = vec![2, 1, 5, 1, 1];
        assert_eq!(3, Solution::max_selected_elements(nums));
    }

    #[test]
    fn test_max_selected_elements_2() {
        let nums: Vec<i32> = vec![1, 4, 7, 10];
        assert_eq!(1, Solution::max_selected_elements(nums));
    }
}
