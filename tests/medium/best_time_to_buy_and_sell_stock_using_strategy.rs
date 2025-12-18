// 3652. Best Time to Buy and Sell Stock using Strategy
// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-using-strategy/

struct Solution;

impl Solution {
    // https://leetcode.com/problems/best-time-to-buy-and-sell-stock-using-strategy/editorial/
    pub fn max_profit(prices: Vec<i32>, strategy: Vec<i32>, k: i32) -> i64 {
        let n = prices.len();
        let mut profit_sum = vec![0i64; n + 1];
        let mut price_sum = vec![0i64; n + 1];
        for i in 0..n {
            profit_sum[i + 1] = profit_sum[i] + prices[i] as i64 * strategy[i] as i64;
            price_sum[i + 1] = price_sum[i] + prices[i] as i64;
        }
        let mut res = profit_sum[n];
        for i in (k - 1) as usize..n {
            let left_profit = profit_sum[i - (k as usize) + 1];
            let right_profit = profit_sum[n] - profit_sum[i + 1];
            let change_profit = price_sum[i + 1] - price_sum[i - (k as usize) / 2 + 1];
            res = res.max(left_profit + change_profit + right_profit);
        }
        res
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

    #[test]
    fn test_max_profit_3() {
        let prices = [5, 8].to_vec();
        let strategy = [-1, -1].to_vec();
        let k = 2;
        assert_eq!(8, Solution::max_profit(prices, strategy, k));
    }

    #[test]
    fn test_max_profit_4() {
        let prices = [8, 5].to_vec();
        let strategy = [1, -1].to_vec();
        let k = 2;
        assert_eq!(5, Solution::max_profit(prices, strategy, k));
    }
}
