// 3652. Best Time to Buy and Sell Stock using Strategy
// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-using-strategy/

struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>, strategy: Vec<i32>, k: i32) -> i64 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::best_time_to_buy_and_sell_stock_using_strategy::Solution;

    #[test]
    fn test_max_profit_1() {
        let prices = [4, 2, 8].to_vec();
        let strategy = [-1, 0, 1].to_vec();
        let k = 2;
        assert_eq!(10, Solution::max_profit(prices, strategy, k));
    }

    #[test]
    fn test_max_profit_2() {
        let prices = [5, 4, 3].to_vec();
        let strategy = [1, 1, 0].to_vec();
        let k = 2;
        assert_eq!(9, Solution::max_profit(prices, strategy, k));
    }
}
