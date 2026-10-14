// 3723. Maximize Sum of Squares of Digits
// https://leetcode.com/problems/maximize-sum-of-squares-of-digits/

struct Solution;

impl Solution {
    pub fn max_sum_of_squares(num: i32, sum: i32) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::maximize_sum_of_squares_of_digits::Solution;

    #[test]
    fn test_max_sum_of_squares_1() {
        let num = 2;
        let sum = 3;
        assert_eq!("30".to_string(), Solution::max_sum_of_squares(num, sum));
    }

    #[test]
    fn test_max_sum_of_squares_2() {
        let num = 2;
        let sum = 17;
        assert_eq!("98".to_string(), Solution::max_sum_of_squares(num, sum));
    }

    #[test]
    fn test_max_sum_of_squares_3() {
        let num = 1;
        let sum = 10;
        assert_eq!("".to_string(), Solution::max_sum_of_squares(num, sum));
    }
}
