// 3754. Concatenate Non-Zero Digits and Multiply by Sum I
// https://leetcode.com/problems/concatenate-non-zero-digits-and-multiply-by-sum-i/

struct Solution;

impl Solution {
    pub fn sum_and_multiply(n: i32) -> i64 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::concatenate_non_zero_digits_and_multiply_by_sum_i::Solution;

    #[test]
    fn test_sum_and_multiply_1() {
        let n = 10203004;
        assert_eq!(12340, Solution::sum_and_multiply(n));
    }

    #[test]
    fn test_sum_and_multiply_2() {
        let n = 1000;
        assert_eq!(1, Solution::sum_and_multiply(n));
    }
}
