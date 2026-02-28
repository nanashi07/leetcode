// 1680. Concatenation of Consecutive Binary Numbers
// https://leetcode.com/problems/concatenation-of-consecutive-binary-numbers/

struct Solution;

impl Solution {
    pub fn concatenated_binary(n: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let mut result: i64 = 0;
        for i in 1..=n as i64 {
            let bit_len = 64 - i.leading_zeros() as i64;
            result = ((result << bit_len) | i) % MOD;
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::concatenation_of_consecutive_binary_numbers::Solution;

    #[test]
    fn test_concatenated_binary_1() {
        let n = 1;
        assert_eq!(1, Solution::concatenated_binary(n));
    }

    #[test]
    fn test_concatenated_binary_2() {
        let n = 3;
        assert_eq!(27, Solution::concatenated_binary(n));
    }

    #[test]
    fn test_concatenated_binary_12() {
        let n = 12;
        assert_eq!(505379714, Solution::concatenated_binary(n));
    }
}
