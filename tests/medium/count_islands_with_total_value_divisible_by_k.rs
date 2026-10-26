// 3619. Count Islands With Total Value Divisible by K
// https://leetcode.com/problems/count-islands-with-total-value-divisible-by-k/

struct Solution;

const DIRS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

impl Solution {
    /// O(rows * cols) time, O(rows * cols) worst-case stack space.
    ///
    /// Flood fill every non-zero region with an explicit stack (the grid can be
    /// 10^3 x 10^3, deep enough to blow the call stack with recursion). Cells are
    /// zeroed when pushed so each one is summed exactly once, and the running sum
    /// is kept in i64 because an island can hold up to 10^6 cells of value 10^6.
    pub fn count_islands(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut grid = grid;
        let rows = grid.len();
        let cols = grid[0].len();
        let k = i64::from(k);

        let mut count = 0;
        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] == 0 {
                    continue;
                }

                // Flood fill this island, sinking it as we go (visited == 0).
                let mut sum = i64::from(grid[r][c]);
                grid[r][c] = 0;
                let mut stack = vec![(r, c)];
                while let Some((y, x)) = stack.pop() {
                    for (dy, dx) in DIRS {
                        let ny = y as i32 + dy;
                        let nx = x as i32 + dx;
                        if ny < 0 || nx < 0 || ny as usize >= rows || nx as usize >= cols {
                            continue;
                        }
                        let (ny, nx) = (ny as usize, nx as usize);
                        if grid[ny][nx] != 0 {
                            sum += i64::from(grid[ny][nx]);
                            grid[ny][nx] = 0;
                            stack.push((ny, nx));
                        }
                    }
                }

                count += i32::from(sum % k == 0);
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::count_islands_with_total_value_divisible_by_k::Solution;
    use crate::shared::vec2d::to_vec2d;

    #[test]
    fn test_count_islands_1() {
        let grid = to_vec2d([
            [0, 2, 1, 0, 0],
            [0, 5, 0, 0, 5],
            [0, 0, 1, 0, 0],
            [0, 1, 4, 7, 0],
            [0, 2, 0, 0, 8],
        ]);
        let k = 5;
        assert_eq!(2, Solution::count_islands(grid, k));
    }

    #[test]
    fn test_count_islands_2() {
        let grid = to_vec2d([[3, 0, 3, 0], [0, 3, 0, 3], [3, 0, 3, 0]]);
        let k = 3;
        assert_eq!(6, Solution::count_islands(grid, k));
    }
}
