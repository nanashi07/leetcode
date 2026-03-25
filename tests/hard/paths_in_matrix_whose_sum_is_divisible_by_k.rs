// 2435. Paths in Matrix Whose Sum Is Divisible by K
// https://leetcode.com/problems/paths-in-matrix-whose-sum-is-divisible-by-k/

struct Solution;

impl Solution {
    pub fn number_of_paths(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        const MOD: i32 = 1_000_000_007;
        let m = grid.len();
        let n = grid[0].len();
        let k = k as usize;

        // dp[i][j][rem] = number of paths to (i,j) with sum % k == rem
        let mut dp = vec![vec![vec![0i64; k]; n]; m];

        // Initialize starting position
        dp[0][0][(grid[0][0] as usize) % k] = 1;

        // Fill the DP table
        for i in 0..m {
            for j in 0..n {
                for rem in 0..k {
                    if dp[i][j][rem] == 0 {
                        continue;
                    }

                    // Move right
                    if j + 1 < n {
                        let new_rem = (rem + grid[i][j + 1] as usize) % k;
                        dp[i][j + 1][new_rem] =
                            (dp[i][j + 1][new_rem] + dp[i][j][rem]) % MOD as i64;
                    }

                    // Move down
                    if i + 1 < m {
                        let new_rem = (rem + grid[i + 1][j] as usize) % k;
                        dp[i + 1][j][new_rem] =
                            (dp[i + 1][j][new_rem] + dp[i][j][rem]) % MOD as i64;
                    }
                }
            }
        }

        dp[m - 1][n - 1][0] as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::paths_in_matrix_whose_sum_is_divisible_by_k::Solution;
    use crate::shared::vec2d::to_vec2d;

    #[test]
    fn test_number_of_paths_1() {
        let grid = to_vec2d([[5, 2, 4], [3, 0, 5], [0, 7, 2]]);
        let k = 3;
        assert_eq!(2, Solution::number_of_paths(grid, k));
    }

    #[test]
    fn test_number_of_paths_2() {
        let grid = to_vec2d([[0, 0]]);
        let k = 5;
        assert_eq!(1, Solution::number_of_paths(grid, k));
    }

    #[test]
    fn test_number_of_paths_3() {
        let grid = to_vec2d([[7, 3, 4, 9], [2, 3, 6, 2], [2, 3, 7, 0]]);
        let k = 1;
        assert_eq!(10, Solution::number_of_paths(grid, k));
    }
}
