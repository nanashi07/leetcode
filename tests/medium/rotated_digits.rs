// 788. Rotated Digits
// https://leetcode.com/problems/rotated-digits/

struct Solution;

impl Solution {
    pub fn rotated_digits(n: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::rotated_digits::Solution;

    #[test]
    fn test_rotated_digits_1() {
        let n = 10;
        assert_eq!(4, Solution::rotated_digits(n));
    }

    #[test]
    fn test_rotated_digits_2() {
        let n = 1;
        assert_eq!(0, Solution::rotated_digits(n));
    }

    #[test]
    fn test_rotated_digits_3() {
        let n = 2;
        assert_eq!(1, Solution::rotated_digits(n));
    }
}
