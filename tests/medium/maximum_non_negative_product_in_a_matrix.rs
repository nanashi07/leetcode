// 1594. Maximum Non Negative Product in a Matrix
// https://leetcode.com/problems/maximum-non-negative-product-in-a-matrix/

struct Solution;

impl Solution {
    pub fn max_product_path(grid: Vec<Vec<i32>>) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let m = grid.len();
        let n = grid[0].len();

        // dp tracks (max_product, min_product) for each cell
        let mut dp = vec![vec![(0i64, 0i64); n]; m];
        dp[0][0] = (grid[0][0] as i64, grid[0][0] as i64);

        for j in 1..n {
            let v = grid[0][j] as i64;
            let (prev_max, prev_min) = dp[0][j - 1];
            dp[0][j] = (prev_max * v, prev_min * v);
            if dp[0][j].0 < dp[0][j].1 {
                dp[0][j] = (dp[0][j].1, dp[0][j].0);
            }
        }

        for i in 1..m {
            let v = grid[i][0] as i64;
            let (prev_max, prev_min) = dp[i - 1][0];
            dp[i][0] = (prev_max * v, prev_min * v);
            if dp[i][0].0 < dp[i][0].1 {
                dp[i][0] = (dp[i][0].1, dp[i][0].0);
            }
        }

        for i in 1..m {
            for j in 1..n {
                let v = grid[i][j] as i64;
                let (up_max, up_min) = dp[i - 1][j];
                let (left_max, left_min) = dp[i][j - 1];
                let candidates = [up_max * v, up_min * v, left_max * v, left_min * v];
                dp[i][j] = (
                    *candidates.iter().max().unwrap(),
                    *candidates.iter().min().unwrap(),
                );
            }
        }

        let (max_prod, _) = dp[m - 1][n - 1];
        if max_prod < 0 {
            -1
        } else {
            (max_prod % MOD) as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::maximum_non_negative_product_in_a_matrix::Solution;
    use crate::shared::vec2d::to_vec2d;

    #[test]
    fn test_max_product_path_1() {
        let grid = to_vec2d([[-1, -2, -3], [-2, -3, -3], [-3, -3, -2]]);
        assert_eq!(-1, Solution::max_product_path(grid));
    }

    #[test]
    fn test_max_product_path_2() {
        let grid = to_vec2d([[1, -2, 1], [1, -2, 1], [3, -4, 1]]);
        assert_eq!(8, Solution::max_product_path(grid));
    }

    #[test]
    fn test_max_product_path_3() {
        let grid = to_vec2d([[1, 3], [0, -4]]);
        assert_eq!(0, Solution::max_product_path(grid));
    }
}
