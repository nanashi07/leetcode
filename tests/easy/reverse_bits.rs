// 190. Reverse Bits
// https://leetcode.com/problems/reverse-bits/

struct Solution;

impl Solution {
    pub fn reverse_bits(n: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::reverse_bits::Solution;

    #[test]
    fn test_reverse_bits_1() {
        let n = 43261596;
        assert_eq!(964176192, Solution::reverse_bits(n));
    }

    #[test]
    fn test_reverse_bits_2() {
        let n = 2147483644;
        assert_eq!(1073741822, Solution::reverse_bits(n));
    }
}
