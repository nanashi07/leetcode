// 1611. Minimum One Bit Operations to Make Integers Zero
// https://leetcode.com/problems/minimum-one-bit-operations-to-make-integers-zero/

struct Solution;

impl Solution {
    pub fn minimum_one_bit_operations(n: i32) -> i32 {
        todo!()
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
