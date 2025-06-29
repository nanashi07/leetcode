// # 1498. Number of Subsequences That Satisfy the Given Sum Condition
// https://leetcode.com/problems/number-of-subsequences-that-satisfy-the-given-sum-condition/

struct Solution;

impl Solution {
    pub fn num_subseq(nums: Vec<i32>, target: i32) -> i32 {
        let modulus: i32 = 10_i32.pow(9) + 7;

        // sorted by hint, but not quite sure why since it should be subsequences
        let mut nums_copy = nums;
        nums_copy.sort();

        let mut power_of_2 = vec![1; nums_copy.len()];
        for i in 1..nums_copy.len() {
            power_of_2[i] = (power_of_2[i - 1] * 2) % modulus;
        }

        let mut left = 0;
        let mut right = nums_copy.len() as i32 - 1; // possible minus so cannot be usize type
        let mut count: i32 = 0;

        while left <= right {
            if nums_copy[left as usize] + nums_copy[right as usize] <= target {
                count += power_of_2[(right - left) as usize];
                count = count % modulus;
                left += 1;
            } else {
                right -= 1;
            }
        }

        count
    }

    /// ref: https://leetcode.com/problems/number-of-subsequences-that-satisfy-the-given-sum-condition/solutions/5418590/rust-solution
    pub fn _num_subseq(nums: Vec<i32>, target: i32) -> i32 {
        const MOD: i32 = 1_000_000_007;
        let mut nums = nums;
        nums.sort();
        let mut ans = 0;

        let mut l = 0;
        let mut r = nums.len() as i32 - 1;

        let mut power_of_2 = vec![1; nums.len()];
        for i in 1..nums.len() {
            power_of_2[i] = (power_of_2[i - 1] * 2) % MOD;
        }

        while l <= r {
            if nums[l as usize] + nums[r as usize] <= target {
                ans = (ans + power_of_2[(r - l) as usize]) % MOD;
                l += 1;
            } else {
                r -= 1;
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::number_of_subsequences_that_satisfy_the_given_sum_condition::Solution;
    #[test]
    fn test_num_subseq_1() {
        let nums = vec![3, 5, 6, 7];
        let target = 9;
        assert_eq!(4, Solution::num_subseq(nums, target));
    }

    #[test]
    fn test_num_subseq_2() {
        let nums = vec![3, 3, 6, 8];
        let target = 10;
        assert_eq!(6, Solution::num_subseq(nums, target));
    }

    #[test]
    fn test_num_subseq_3() {
        let nums = vec![2, 3, 3, 4, 6, 7];
        let target = 12;
        assert_eq!(61, Solution::num_subseq(nums, target));
    }

    #[test]
    fn test_num_subseq_4() {
        let nums = vec![7, 10, 7, 3, 7, 5, 4];
        let target = 12;
        assert_eq!(56, Solution::num_subseq(nums, target));
    }

    #[test]
    fn test_num_subseq_5() {
        let nums = vec![
            14, 4, 6, 6, 20, 8, 5, 6, 8, 12, 6, 10, 14, 9, 17, 16, 9, 7, 14, 11, 14, 15, 13, 11,
            10, 18, 13, 17, 17, 14, 17, 7, 9, 5, 10, 13, 8, 5, 18, 20, 7, 5, 5, 15, 19, 14,
        ];
        let target = 22;
        assert_eq!(272187084, Solution::num_subseq(nums, target));
    }

    #[test]
    fn test_num_subseq_6() {
        let nums = vec![
            9, 25, 9, 28, 24, 12, 17, 8, 28, 7, 21, 25, 10, 2, 16, 19, 12, 13, 15, 28, 14, 12, 24,
            9, 6, 7, 2, 15, 19, 13, 30, 30, 23, 19, 11, 3, 17, 2, 14, 20, 22, 30, 12, 1, 11, 2, 2,
            20, 20, 27, 15, 9, 10, 4, 12, 30, 13, 5, 2, 11, 29, 5, 3, 13, 22, 5, 16, 19, 7, 19, 11,
            16, 11, 25, 29, 21, 29, 3, 2, 9, 20, 15, 9,
        ];
        let target = 32;
        assert_eq!(91931447, Solution::num_subseq(nums, target));
    }

    #[test]
    fn test_num_subseq_7() {
        let nums = vec![1];
        let target = 1;
        assert_eq!(0, Solution::num_subseq(nums, target));
    }
}
