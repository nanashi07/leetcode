// 3483. Unique 3-Digit Even Numbers
// https://leetcode.com/problems/unique-3-digit-even-numbers/

struct Solution;

impl Solution {
    pub fn total_numbers(digits: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::unique_3_digit_even_numbers::Solution;

    #[test]
    fn test_total_numbers_1() {
        let digits = [1, 2, 3, 4].to_vec();
        assert_eq!(12, Solution::total_numbers(digits));
    }

    #[test]
    fn test_total_numbers_2() {
        let digits = [0, 2, 2].to_vec();
        assert_eq!(2, Solution::total_numbers(digits));
    }

    #[test]
    fn test_total_numbers_3() {
        let digits = [6, 6, 6].to_vec();
        assert_eq!(1, Solution::total_numbers(digits));
    }

    #[test]
    fn test_total_numbers_4() {
        let digits = [1, 3, 5].to_vec();
        assert_eq!(0, Solution::total_numbers(digits));
    }
}
