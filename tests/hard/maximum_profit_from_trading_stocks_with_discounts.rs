// 3562. Maximum Profit from Trading Stocks with Discounts
// https://leetcode.com/problems/maximum-profit-from-trading-stocks-with-discounts/

struct Solution;

impl Solution {
    pub fn max_profit(
        n: i32,
        present: Vec<i32>,
        future: Vec<i32>,
        hierarchy: Vec<Vec<i32>>,
        budget: i32,
    ) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::maximum_profit_from_trading_stocks_with_discounts::Solution;

    #[test]
    fn test_max_profit_1() {
        let n = 2;
        let present = [1, 2].to_vec();
        let future = [4, 3].to_vec();
        let hierarchy = [[1, 2]].iter().map(|l| l.to_vec()).collect::<Vec<_>>();
        let budget = 3;
        assert_eq!(
            5,
            Solution::max_profit(n, present, future, hierarchy, budget)
        );
    }

    #[test]
    fn test_max_profit_2() {
        let n = 2;
        let present = [3, 4].to_vec();
        let future = [5, 8].to_vec();
        let hierarchy = [[1, 2]].iter().map(|l| l.to_vec()).collect::<Vec<_>>();
        let budget = 4;
        assert_eq!(
            4,
            Solution::max_profit(n, present, future, hierarchy, budget)
        );
    }

    #[test]
    fn test_max_profit_3() {
        let n = 3;
        let present = [4, 6, 8].to_vec();
        let future = [7, 9, 11].to_vec();
        let hierarchy = [[1, 2], [1, 3]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        let budget = 10;
        assert_eq!(
            10,
            Solution::max_profit(n, present, future, hierarchy, budget)
        );
    }

    #[test]
    fn test_max_profit_4() {
        let n = 3;
        let present = [5, 2, 3].to_vec();
        let future = [8, 5, 6].to_vec();
        let hierarchy = [[1, 2], [2, 3]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        let budget = 7;
        assert_eq!(
            12,
            Solution::max_profit(n, present, future, hierarchy, budget)
        );
    }
}
