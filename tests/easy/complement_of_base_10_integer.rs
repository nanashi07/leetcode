// 1009. Complement of Base 10 Integer
// https://leetcode.com/problems/complement-of-base-10-integer/

struct Solution;

impl Solution {
    pub fn bitwise_complement(n: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::complement_of_base_10_integer::Solution;

    #[test]
    fn test_bitwise_complement_1() {
        let n = 5;
        assert_eq!(2, Solution::bitwise_complement(n));
    }

    #[test]
    fn test_bitwise_complement_2() {
        let n = 7;
        assert_eq!(0, Solution::bitwise_complement(n));
    }

    #[test]
    fn test_bitwise_complement_3() {
        let n = 10;
        assert_eq!(5, Solution::bitwise_complement(n));
    }
}
