// 2110. Number of Smooth Descent Periods of a Stock
// https://leetcode.com/problems/number-of-smooth-descent-periods-of-a-stock/

struct Solution;

impl Solution {
    pub fn get_descent_periods(prices: Vec<i32>) -> i64 {
        println!("prices: {:?}", &prices);

        let mut start = 0;
        let mut sec = vec![];

        for i in 1..prices.len() {
            let p = prices[i];
            if p == prices[i - 1] - 1 {
                // go next
                if i == prices.len() - 1 && i - start > 0 {
                    sec.push((i - start + 1) as i64);
                }
            } else {
                if i - start > 1 {
                    sec.push((i - start) as i64);
                }
                start = i;
            }
        }

        println!("sec: {:?}", &sec);

        sec.iter().map(|s| (s + 1) * s / 2 - s).sum::<i64>() + prices.len() as i64
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

    #[test]
    fn test_get_descent_periods_4() {
        let prices = [12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 4, 3, 10, 9, 8, 7].to_vec();
        assert_eq!(68, Solution::get_descent_periods(prices));
    }
}
