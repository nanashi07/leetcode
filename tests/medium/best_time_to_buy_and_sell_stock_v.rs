// 3573. Best Time to Buy and Sell Stock V
// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-v/

struct Solution;

impl Solution {
    pub fn maximum_profit(prices: Vec<i32>, k: i32) -> i64 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::best_time_to_buy_and_sell_stock_v::Solution;

    #[test]
    fn test_maximum_profit_1() {
        let prices = [1, 7, 9, 8, 2].to_vec();
        let k = 2;
        assert_eq!(14, Solution::maximum_profit(prices, k));
    }

    #[test]
    fn test_maximum_profit_2() {
        let prices = [12, 16, 19, 19, 8, 1, 19, 13, 9].to_vec();
        let k = 3;
        assert_eq!(36, Solution::maximum_profit(prices, k));
    }
}
