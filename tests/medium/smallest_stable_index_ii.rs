// 3904. Smallest Stable Index II
// https://leetcode.com/problems/smallest-stable-index-ii/

struct Solution;

impl Solution {
    pub fn first_stable_index(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        if n == 0 {
            return -1;
        }
        let mut min_suffix = vec![0; n];
        min_suffix[n - 1] = nums[n - 1];
        for i in (0..n - 1).rev() {
            min_suffix[i] = min_suffix[i + 1].min(nums[i]);
        }
        let mut max_prefix = i32::MIN;
        for (i, &num) in nums.iter().enumerate() {
            max_prefix = max_prefix.max(num);
            if max_prefix - min_suffix[i] <= k {
                return i as i32;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::smallest_stable_index_ii::Solution;

    #[test]
    fn test_first_stable_index_1() {
        let nums = [5, 0, 1, 4].to_vec();
        let k = 3;
        assert_eq!(3, Solution::first_stable_index(nums, k));
    }

    #[test]
    fn test_first_stable_index_2() {
        let nums = [3, 2, 1].to_vec();
        let k = 1;
        assert_eq!(-1, Solution::first_stable_index(nums, k));
    }

    #[test]
    fn test_first_stable_index_3() {
        let nums = [0].to_vec();
        let k = 0;
        assert_eq!(0, Solution::first_stable_index(nums, k));
    }
}
