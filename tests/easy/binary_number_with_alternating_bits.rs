// 693. Binary Number with Alternating Bits
// https://leetcode.com/problems/binary-number-with-alternating-bits/

struct Solution;

impl Solution {
    pub fn has_alternating_bits(n: i32) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::binary_number_with_alternating_bits::Solution;

    #[test]
    fn test_has_alternating_bits_1() {
        let n = 5;
        assert_eq!(true, Solution::has_alternating_bits(n));
    }

    #[test]
    fn test_has_alternating_bits_2() {
        let n = 7;
        assert_eq!(false, Solution::has_alternating_bits(n));
    }

    #[test]
    fn test_has_alternating_bits_3() {
        let n = 11;
        assert_eq!(false, Solution::has_alternating_bits(n));
    }
}
