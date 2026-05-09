// 1914. Cyclically Rotating a Grid
// https://leetcode.com/problems/cyclically-rotating-a-grid/

struct Solution;

impl Solution {
    pub fn rotate_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let rows = grid.len();
        let cols = grid[0].len();
        let mut grid = grid;

        let layers = rows.min(cols) / 2;
        for layer in 0..layers {
            let top = layer;
            let bottom = rows - 1 - layer;
            let left = layer;
            let right = cols - 1 - layer;

            let mut coords = Vec::with_capacity(2 * (bottom - top + right - left));

            for c in left..right {
                coords.push((top, c));
            }
            for r in top..bottom {
                coords.push((r, right));
            }
            for c in (left + 1..=right).rev() {
                coords.push((bottom, c));
            }
            for r in (top + 1..=bottom).rev() {
                coords.push((r, left));
            }

            let len = coords.len();
            let shift = (k as usize) % len;
            if shift == 0 {
                continue;
            }

            let vals: Vec<i32> = coords.iter().map(|&(r, c)| grid[r][c]).collect();
            for (i, &(r, c)) in coords.iter().enumerate() {
                grid[r][c] = vals[(i + shift) % len];
            }
        }

        grid
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::cyclically_rotating_a_grid::Solution;
    use crate::shared::vec2d::to_vec2d;

    #[test]
    fn test_rotate_grid_1() {
        let grid = to_vec2d([[40, 10], [30, 20]]);
        let k = 1;
        let output = to_vec2d([[10, 20], [40, 30]]);
        assert_eq!(output, Solution::rotate_grid(grid, k));
    }

    #[test]
    fn test_rotate_grid_2() {
        let grid = to_vec2d([
            [1, 2, 3, 4],
            [5, 6, 7, 8],
            [9, 10, 11, 12],
            [13, 14, 15, 16],
        ]);
        let k = 2;
        let output = to_vec2d([
            [3, 4, 8, 12],
            [2, 11, 10, 16],
            [1, 7, 6, 15],
            [5, 9, 13, 14],
        ]);
        assert_eq!(output, Solution::rotate_grid(grid, k));
    }
}
