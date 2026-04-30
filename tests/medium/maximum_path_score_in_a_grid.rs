// 3742. Maximum Path Score in a Grid
// https://leetcode.com/problems/maximum-path-score-in-a-grid/

struct Solution;

impl Solution {
    pub fn max_path_score(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let k = k as usize;
        let inf = 1 << 30;

        // dp[i][j][c] = max score reaching (i,j) with cost c
        let mut dp = vec![vec![vec![-inf; k + 1]; n]; m];
        dp[0][0][0] = 0;

        for i in 0..m {
            for j in 0..n {
                if i == 0 && j == 0 {
                    continue;
                }
                let val = grid[i][j];
                let cost = if val > 0 { 1 } else { 0 };
                for c in cost..=k {
                    let from_top = if i > 0 { dp[i - 1][j][c - cost] } else { -inf };
                    let from_left = if j > 0 { dp[i][j - 1][c - cost] } else { -inf };
                    let best = from_top.max(from_left);
                    if best > -inf {
                        dp[i][j][c] = dp[i][j][c].max(best + val);
                    }
                }
            }
        }

        let ans = *dp[m - 1][n - 1].iter().max().unwrap();
        if ans < 0 { -1 } else { ans }
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::maximum_path_score_in_a_grid::Solution;
    use crate::shared::vec2d::to_vec2d;

    #[test]
    fn test_max_path_score_1() {
        let grid = to_vec2d([[0, 1], [2, 0]]);
        let k = 1;
        assert_eq!(2, Solution::max_path_score(grid, k));
    }

    #[test]
    fn test_max_path_score_2() {
        let grid = to_vec2d([[0, 1], [1, 2]]);
        let k = 1;
        assert_eq!(-1, Solution::max_path_score(grid, k));
    }
}
