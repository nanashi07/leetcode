// 3665. Twisted Mirror Path Count
// https://leetcode.com/problems/twisted-mirror-path-count/

struct Solution;

impl Solution {
    /// Walks from `(0, 0)` to the bottom-right corner moving only right or down. A cell
    /// holding `1` is a twisted mirror: leaving it you must turn, i.e. your next move has to
    /// be perpendicular to the move that carried you in. Empty cells (`0`) let you either go
    /// straight or turn; mirrors are passable, they only constrain the direction.
    ///
    /// `dp[d]` counts walks reaching a cell whose last move was `d`, so a mirror predecessor
    /// contributes only its opposite-direction count. Rolling rows keep this O(m * n) time and
    /// O(n) space.
    pub fn unique_paths(grid: Vec<Vec<i32>>) -> i32 {
        // Direction of the move that entered a cell.
        const DOWN: usize = 0;
        const RIGHT: usize = 1;

        let m = grid.len();
        let n = grid[0].len();
        if m == 1 && n == 1 {
            return 1;
        }

        let mut above = vec![[0i64; 2]; n];
        let mut row = vec![[0i64; 2]; n];
        for r in 0..m {
            for cell in row.iter_mut() {
                *cell = [0, 0];
            }
            for c in 0..n {
                // Arrive at (r, c) moving down from (r - 1, c); the first move is free.
                if r > 0 {
                    row[c][DOWN] = if r == 1 && c == 0 {
                        1
                    } else if grid[r - 1][c] == 1 {
                        above[c][RIGHT]
                    } else {
                        above[c][DOWN] + above[c][RIGHT]
                    };
                }
                // Arrive at (r, c) moving right from (r, c - 1).
                if c > 0 {
                    row[c][RIGHT] = if r == 0 && c == 1 {
                        1
                    } else if grid[r][c - 1] == 1 {
                        row[c - 1][DOWN]
                    } else {
                        row[c - 1][DOWN] + row[c - 1][RIGHT]
                    };
                }
            }
            std::mem::swap(&mut above, &mut row);
        }

        (above[n - 1][DOWN] + above[n - 1][RIGHT]) as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::twisted_mirror_path_count::Solution;
    use crate::shared::vec2d::to_vec2d;

    #[test]
    fn test_unique_paths_1() {
        let grid = to_vec2d([[0, 1, 0], [0, 0, 1], [1, 0, 0]]);
        assert_eq!(5, Solution::unique_paths(grid));
    }

    #[test]
    fn test_unique_paths_2() {
        let grid = to_vec2d([[0, 0], [0, 0]]);
        assert_eq!(2, Solution::unique_paths(grid));
    }

    #[test]
    fn test_unique_paths_3() {
        let grid = to_vec2d([[0, 1, 1], [1, 1, 0]]);
        assert_eq!(1, Solution::unique_paths(grid));
    }
}
