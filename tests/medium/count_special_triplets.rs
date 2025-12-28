// 3583. Count Special Triplets
// https://leetcode.com/problems/count-special-triplets/

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn special_triplets(nums: Vec<i32>) -> i32 {
        const MOD: i64 = 1_000_000_007;

        let mut left: HashMap<i32, i64> = HashMap::new();
        let mut right: HashMap<i32, i64> = HashMap::new();

        // Initialize right map with all elements
        for &num in &nums {
            *right.entry(num).or_insert(0) += 1;
        }

        let mut ans: i64 = 0;

        // Enumerate each position as the middle element j
        for &x in &nums {
            // Remove current element from right
            *right.get_mut(&x).unwrap() -= 1;

            // Count special triplets with x as middle element
            // We need nums[i] = 2*x and nums[k] = 2*x
            let target = x * 2;
            let left_count = *left.get(&target).unwrap_or(&0);
            let right_count = *right.get(&target).unwrap_or(&0);

            ans = (ans + (left_count * right_count) % MOD) % MOD;

            // Add current element to left
            *left.entry(x).or_insert(0) += 1;
        }

        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::count_special_triplets::Solution;

    #[test]
    fn test_special_triplets_1() {
        let nums = [6, 3, 6].to_vec();
        assert_eq!(1, Solution::special_triplets(nums));
    }

    #[test]
    fn test_special_triplets_2() {
        let nums = [0, 1, 0, 0].to_vec();
        assert_eq!(1, Solution::special_triplets(nums));
    }

    #[test]
    fn test_special_triplets_3() {
        let nums = [8, 4, 2, 8, 4].to_vec();
        assert_eq!(2, Solution::special_triplets(nums));
    }
}
