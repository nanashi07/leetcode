// 2267. Check if There Is a Valid Parentheses String Path
// https://leetcode.com/problems/check-if-there-is-a-valid-parentheses-string-path/

struct Solution;

impl Solution {
    pub fn has_valid_path(grid: Vec<Vec<char>>) -> bool {
        let m = grid.len();
        let n = grid[0].len();
        // Every path from (0, 0) to (m - 1, n - 1) reads exactly `m + n - 1` cells and
        // only moves right/down, so the length must be even and the endpoints are fixed.
        if (m + n).is_multiple_of(2) || grid[0][0] != '(' || grid[m - 1][n - 1] != ')' {
            return false;
        }

        // `dp[j]` is a bitset: bit `b` is set when the balance (number of still unclosed
        // '(') `b` is reachable at cell `(i, j)`. A balance larger than the number of
        // remaining steps can never be closed again, so with `m, n <= 100` the highest
        // bit ever needed is `floor((m + n - 2) / 2) <= 99`, which fits in a `u128`.
        let mut dp = vec![0_u128; n];
        for (i, row) in grid.iter().enumerate() {
            // Nothing comes from the left of a new row.
            let mut left = 0_u128;
            for j in 0..n {
                let up = dp[j];
                // Before the first cell the balance is 0, which is the only seed state.
                let prev = if i == 0 && j == 0 { 1 } else { up | left };
                // '(' raises the balance, ')' lowers it; shifting right drops bit 0, so a
                // path that would close more brackets than it opened dies out here.
                let mut mask = if row[j] == '(' { prev << 1 } else { prev >> 1 };
                let rest = (m - 1 - i) + (n - 1 - j);
                mask &= if rest >= 127 {
                    u128::MAX
                } else {
                    (1_u128 << (rest + 1)) - 1
                };
                dp[j] = mask;
                left = mask;
            }
        }

        // At the last cell only balance 0 survives the pruning above.
        dp[n - 1] != 0
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::check_if_there_is_a_valid_parentheses_string_path::Solution;
    use crate::shared::vec2d::{to_char_vec2d, to_string_vec2d};

    #[test]
    fn test_has_valid_path_1() {
        let grid = to_char_vec2d([
            ["(", "(", "("],
            [")", "(", ")"],
            ["(", "(", ")"],
            ["(", "(", ")"],
        ]);
        assert!(Solution::has_valid_path(grid));
    }

    #[test]
    fn test_has_valid_path_2() {
        let grid = to_char_vec2d([
            ["(", "(", "("],
            [")", "(", ")"],
            ["(", "(", ")"],
            ["(", "(", ")"],
        ]);
        assert!(Solution::has_valid_path(grid));
    }
}
