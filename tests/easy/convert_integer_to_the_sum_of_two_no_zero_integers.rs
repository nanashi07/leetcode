// # 1317. Convert Integer to the Sum of Two No-Zero Integers
// https://leetcode.com/problems/convert-integer-to-the-sum-of-two-no-zero-integers/

struct Solution;

impl Solution {
    pub fn get_no_zero_integers(n: i32) -> Vec<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::convert_integer_to_the_sum_of_two_no_zero_integers::Solution;

    #[test]
    fn test_get_no_zero_integers_1() {
        let n = 2;
        assert_eq!([1, 1].to_vec(), Solution::get_no_zero_integers(n));
    }

    #[test]
    fn test_get_no_zero_integers_2() {
        let n = 11;
        assert_eq!([2, 9].to_vec(), Solution::get_no_zero_integers(n));
    }
}
