// 3754. Concatenate Non-Zero Digits and Multiply by Sum I
// https://leetcode.com/problems/concatenate-non-zero-digits-and-multiply-by-sum-i/

struct Solution;

impl Solution {
    pub fn sum_and_multiply(n: i32) -> i64 {
        let mut x = 0;
        let mut sum = 0;
        let mut n = n as i64;
        let mut c = 0;
        while n > 0 {
            let s = n % 10;
            if s > 0 {
                x = x + s * 10i64.pow(c);
                sum = sum + s;
                c += 1;
            }
            n = n / 10
        }
        x * sum
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
