// 3418. Maximum Amount of Money Robot Can Earn
// https://leetcode.com/problems/maximum-amount-of-money-robot-can-earn/

struct Solution;

impl Solution {
    pub fn maximum_amount(coins: Vec<Vec<i32>>) -> i32 {
        let m = coins.len();
        let n = coins[0].len();
        const NEG_INF: i32 = i32::MIN / 2;

        // dp[i][j][k] = max coins collectible at cell (i,j) having used k neutralizations
        let mut dp = vec![vec![[NEG_INF; 3]; n]; m];

        let apply = |dp: &mut Vec<Vec<[i32; 3]>>, i: usize, j: usize, prev: i32, k: usize| {
            let v = coins[i][j];
            dp[i][j][k] = dp[i][j][k].max(prev + v);
            if v < 0 && k < 2 {
                dp[i][j][k + 1] = dp[i][j][k + 1].max(prev);
            }
        };

        dp[0][0][0] = coins[0][0];
        if coins[0][0] < 0 {
            dp[0][0][1] = 0;
        }

        for j in 1..n {
            for k in 0..3 {
                let prev = dp[0][j - 1][k];
                if prev != NEG_INF {
                    apply(&mut dp, 0, j, prev, k);
                }
            }
        }

        for i in 1..m {
            for k in 0..3 {
                let prev = dp[i - 1][0][k];
                if prev != NEG_INF {
                    apply(&mut dp, i, 0, prev, k);
                }
            }
        }

        for i in 1..m {
            for j in 1..n {
                for k in 0..3 {
                    let best = dp[i - 1][j][k].max(dp[i][j - 1][k]);
                    if best != NEG_INF {
                        apply(&mut dp, i, j, best, k);
                    }
                }
            }
        }

        *dp[m - 1][n - 1].iter().max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::maximum_amount_of_money_robot_can_earn::Solution;
    use crate::shared::vec2d::to_vec2d;

    #[test]
    fn test_maximum_amount_1() {
        let coins = to_vec2d([[0, 1, -1], [1, -2, 3], [2, -3, 4]]);
        assert_eq!(8, Solution::maximum_amount(coins));
    }

    #[test]
    fn test_maximum_amount_2() {
        let coins = to_vec2d([[10, 10, 10], [10, 10, 10]]);
        assert_eq!(40, Solution::maximum_amount(coins));
    }
}
