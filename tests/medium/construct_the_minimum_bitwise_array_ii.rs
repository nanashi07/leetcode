// 3315. Construct the Minimum Bitwise Array II
// https://leetcode.com/problems/construct-the-minimum-bitwise-array-ii/

struct Solution;

impl Solution {
    pub fn min_bitwise_array(nums: Vec<i32>) -> Vec<i32> {
        nums.into_iter()
            .map(|num| {
                // If num is even, no solution exists
                if num % 2 == 0 {
                    return -1;
                }

                // For odd numbers, we need to find ans such that ans | (ans + 1) = num
                // Key insight: ans | (ans + 1) always sets all bits from the rightmost 0 in ans to the right
                // We need to find the rightmost position where we can clear a bit in num

                // Find the rightmost consecutive 1s and flip the rightmost one
                // Example: 7 = 111, we want 011 = 3, then 3 | 4 = 011 | 100 = 111 = 7
                // Example: 5 = 101, we want 100 = 4, then 4 | 5 = 100 | 101 = 101 = 5
                // Example: 11 = 1011, we want 1001 = 9, then 9 | 10 = 1001 | 1010 = 1011 = 11

                // Find the first position from right where bit transitions from 1 to 0
                let mut ans = num;
                let mut pos = 0;

                // Find rightmost consecutive 1s
                while (ans >> pos) & 1 == 1 {
                    pos += 1;
                }

                // Clear the bit at position (pos - 1)
                ans &= !(1 << (pos - 1));

                ans
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::construct_the_minimum_bitwise_array_ii::Solution;

    #[test]
    fn test_min_bitwise_array_1() {
        let nums = [2, 3, 5, 7].to_vec();
        let output = [-1, 1, 4, 3].to_vec();
        assert_eq!(output, Solution::min_bitwise_array(nums));
    }

    #[test]
    fn test_min_bitwise_array_2() {
        let nums = [11, 13, 31].to_vec();
        let output = [9, 12, 15].to_vec();
        assert_eq!(output, Solution::min_bitwise_array(nums));
    }
}
