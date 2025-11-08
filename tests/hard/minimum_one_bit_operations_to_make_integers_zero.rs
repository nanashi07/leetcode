// 1611. Minimum One Bit Operations to Make Integers Zero
// https://leetcode.com/problems/minimum-one-bit-operations-to-make-integers-zero/

struct Solution;

impl Solution {
    pub fn minimum_one_bit_operations(n: i32) -> i32 {
        if n == 0 {
            return 0;
        }

        // The answer is the inverse Gray code of n
        // We can compute this by XORing n with itself shifted right repeatedly
        let mut result = n;
        let mut shift = n;

        while shift > 0 {
            shift >>= 1;
            result ^= shift;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::minimum_one_bit_operations_to_make_integers_zero::Solution;

    #[test]
    fn test_minimum_one_bit_operations_1() {
        let n = 3;
        assert_eq!(2, Solution::minimum_one_bit_operations(n));
    }

    #[test]
    fn test_minimum_one_bit_operations_2() {
        let n = 6;
        assert_eq!(4, Solution::minimum_one_bit_operations(n));
    }
}
