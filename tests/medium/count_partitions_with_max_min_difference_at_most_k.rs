// 3578. Count Partitions With Max-Min Difference at Most K
// https://leetcode.com/problems/count-partitions-with-max-min-difference-at-most-k/

use std::collections::BTreeMap;

struct Solution;

impl Solution {
    pub fn count_partitions(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mod_val = 1_000_000_007i64;

        // dp[i] = number of ways to partition nums[0..i]
        let mut dp = vec![0i64; n + 1];
        // prefix[i] = sum of dp[0..=i] for quick range sum queries
        let mut prefix = vec![0i64; n + 1];
        // cnt = frequency map of elements in current window
        let mut cnt = BTreeMap::new();

        dp[0] = 1; // base case: empty array has 1 way to partition
        prefix[0] = 1;

        let mut j = 0; // left pointer of sliding window

        for i in 0..n {
            // Add current element to window
            *cnt.entry(nums[i]).or_insert(0) += 1;

            // Shrink window from left while max - min > k
            while j <= i && *cnt.keys().last().unwrap() - *cnt.keys().next().unwrap() > k {
                *cnt.get_mut(&nums[j]).unwrap() -= 1;
                if cnt[&nums[j]] == 0 {
                    cnt.remove(&nums[j]);
                }
                j += 1;
            }

            // dp[i+1] = sum of dp[j..=i]
            // This counts all valid partitions ending at position i
            // where the last segment starts from any position in [j, i]
            if j > 0 {
                dp[i + 1] = (prefix[i] - prefix[j - 1] + mod_val) % mod_val;
            } else {
                dp[i + 1] = prefix[i] % mod_val;
            }

            prefix[i + 1] = (prefix[i] + dp[i + 1]) % mod_val;
        }

        dp[n] as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::count_partitions_with_max_min_difference_at_most_k::Solution;

    #[test]
    fn test_count_partitions_1() {
        let nums = [9, 4, 1, 3, 7].to_vec();
        let k = 4;
        assert_eq!(6, Solution::count_partitions(nums, k));
    }

    #[test]
    fn test_count_partitions_2() {
        let nums = [3, 3, 4].to_vec();
        let k = 0;
        assert_eq!(2, Solution::count_partitions(nums, k));
    }
}
