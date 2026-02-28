// 1680. Concatenation of Consecutive Binary Numbers
// https://leetcode.com/problems/concatenation-of-consecutive-binary-numbers/

struct Solution;

impl Solution {
    pub fn concatenated_binary(n: i32) -> i32 {
        todo!()
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
        let n = 112;
        assert_eq!(505379714, Solution::concatenated_binary(n));
    }
}
