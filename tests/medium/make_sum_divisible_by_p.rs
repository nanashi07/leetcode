// 1590. Make Sum Divisible by P
// https://leetcode.com/problems/make-sum-divisible-by-p/

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
        let n = nums.len();
        let p = p as i64;

        // Calculate total sum mod p
        let total_sum: i64 = nums.iter().map(|&x| x as i64).sum();
        let target = total_sum % p;

        // If already divisible, no need to remove anything
        if target == 0 {
            return 0;
        }

        // HashMap to store (prefix_sum % p) -> index
        let mut mod_map: HashMap<i64, usize> = HashMap::new();
        mod_map.insert(0, 0); // Empty prefix at index 0

        let mut prefix_sum: i64 = 0;
        let mut min_len = n + 1;

        for i in 0..n {
            prefix_sum += nums[i] as i64;
            let current_mod = prefix_sum.rem_euclid(p);

            // We want to find a previous prefix where:
            // (current_mod - prev_mod) % p == target
            // So: prev_mod = (current_mod - target) % p
            let needed_mod = (current_mod - target).rem_euclid(p);

            if let Some(&prev_idx) = mod_map.get(&needed_mod) {
                // Subarray from prev_idx to i (inclusive)
                // Length is (i + 1) - prev_idx
                min_len = min_len.min(i + 1 - prev_idx);
            }

            // Store current prefix mod at position i+1 (after including nums[i])
            mod_map.insert(current_mod, i + 1);
        }

        // Can't remove the entire array
        if min_len >= n {
            -1
        } else {
            min_len as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::make_sum_divisible_by_p::Solution;

    #[test]
    fn test_min_subarray_1() {
        let nums = [3, 1, 4, 2].to_vec();
        let p = 6;
        assert_eq!(1, Solution::min_subarray(nums, p));
    }

    #[test]
    fn test_min_subarray_2() {
        let nums = [6, 3, 5, 2].to_vec();
        let p = 9;
        assert_eq!(2, Solution::min_subarray(nums, p));
    }

    #[test]
    fn test_min_subarray_3() {
        let nums = [1, 2, 3].to_vec();
        let p = 3;
        assert_eq!(0, Solution::min_subarray(nums, p));
    }

    #[test]
    fn test_min_subarray_4() {
        let nums = [1, 2, 3].to_vec();
        let p = 7;
        assert_eq!(-1, Solution::min_subarray(nums, p));
    }
}
