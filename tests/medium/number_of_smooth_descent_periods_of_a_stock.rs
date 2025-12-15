// 2110. Number of Smooth Descent Periods of a Stock
// https://leetcode.com/problems/number-of-smooth-descent-periods-of-a-stock/

struct Solution;

impl Solution {
    pub fn get_descent_periods(prices: Vec<i32>) -> i64 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::number_of_smooth_descent_periods_of_a_stock::Solution;

    #[test]
    fn test_get_descent_periods_1() {
        let prices = [3, 2, 1, 4].to_vec();
        assert_eq!(7, Solution::get_descent_periods(prices));
    }

    #[test]
    fn test_get_descent_periods_2() {
        let prices = [8, 6, 7, 7].to_vec();
        assert_eq!(4, Solution::get_descent_periods(prices));
    }

    #[test]
    fn test_get_descent_periods_3() {
        let prices = [1].to_vec();
        assert_eq!(1, Solution::get_descent_periods(prices));
    }
}
