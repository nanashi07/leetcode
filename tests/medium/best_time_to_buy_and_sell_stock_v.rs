// 3573. Best Time to Buy and Sell Stock V
// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-v/

struct Solution;

impl Solution {
    pub fn maximum_profit(prices: Vec<i32>, k: i32) -> i64 {
        let n = prices.len();
        if n == 0 || k == 0 {
            return 0;
        }

        let k = k as usize;

        // dp[i][j] represents the state after processing day j with at most i transactions
        // State 0: neutral (no position)
        // State 1: holding long position
        // State 2: holding short position
        let mut dp = vec![vec![(0i64, i64::MIN / 2, i64::MIN / 2); n]; k + 1];

        // Initialize: with 0 transactions, we can't hold any position
        for j in 0..n {
            dp[0][j].0 = 0;
        }

        // Initialize day 0 for each transaction count
        for i in 1..=k {
            dp[i][0].1 = -(prices[0] as i64); // buy long
            dp[i][0].2 = prices[0] as i64; // sell short
        }

        for i in 1..=k {
            for j in 1..n {
                let price = prices[j] as i64;

                // Neutral with i transactions completed: either was neutral, or close long/short (completes i-th transaction)
                dp[i][j].0 = dp[i][j - 1]
                    .0
                    .max(dp[i][j - 1].1 + price) // close long position (complete i-th transaction)
                    .max(dp[i][j - 1].2 - price); // close short position (complete i-th transaction)

                // Holding long (i-th transaction in progress): either held long yesterday, or open long from (i-1) completed
                dp[i][j].1 = dp[i][j - 1].1.max(dp[i - 1][j - 1].0 - price);

                // Holding short (i-th transaction in progress): either held short yesterday, or open short from (i-1) completed
                dp[i][j].2 = dp[i][j - 1].2.max(dp[i - 1][j - 1].0 + price);
            }
        }

        dp[k][n - 1].0
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

    #[test]
    fn test_maximum_profit_3() {
        let prices = [14, 6].to_vec();
        let k = 1;
        assert_eq!(8, Solution::maximum_profit(prices, k));
    }
}
