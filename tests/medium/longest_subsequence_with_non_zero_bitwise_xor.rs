// 3702. Longest Subsequence With Non-Zero Bitwise XOR
// https://leetcode.com/problems/longest-subsequence-with-non-zero-bitwise-xor/

struct Solution;

impl Solution {
    pub fn longest_subsequence(nums: Vec<i32>) -> i32 {
        let mut xor = 0;
        let mut all_zero = true;
        for &x in &nums {
            xor ^= x;
            if x != 0 {
                all_zero = false;
            }
        }
        if xor != 0 {
            nums.len() as i32
        } else if all_zero {
            0
        } else {
            (nums.len() - 1) as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::longest_subsequence_with_non_zero_bitwise_xor::Solution;

    #[test]
    fn test_longest_subsequence_1() {
        let nums = [1, 2, 3].to_vec();
        assert_eq!(2, Solution::longest_subsequence(nums));
    }

    #[test]
    fn test_longest_subsequence_2() {
        let nums = [2, 3, 4].to_vec();
        assert_eq!(3, Solution::longest_subsequence(nums));
    }
}
