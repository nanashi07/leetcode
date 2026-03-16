// 1878. Get Biggest Three Rhombus Sums in a Grid
// https://leetcode.com/problems/get-biggest-three-rhombus-sums-in-a-grid/

use std::collections::BTreeSet;

struct Solution;

impl Solution {
    pub fn get_biggest_three(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let rows = grid.len();
        let cols = grid[0].len();

        let mut down_right = vec![vec![0; cols + 1]; rows + 1];
        let mut down_left = vec![vec![0; cols + 1]; rows + 1];

        for row in 0..rows {
            for col in 0..cols {
                down_right[row + 1][col + 1] = down_right[row][col] + grid[row][col];
            }
            for col in (0..cols).rev() {
                down_left[row + 1][col] = down_left[row][col + 1] + grid[row][col];
            }
        }

        let mut best = BTreeSet::new();

        for row in 0..rows {
            for col in 0..cols {
                Self::push_sum(&mut best, grid[row][col]);

                let max_radius = row.min(rows - 1 - row).min(col).min(cols - 1 - col);
                for radius in 1..=max_radius {
                    let top = (row - radius, col);
                    let right = (row, col + radius);
                    let bottom = (row + radius, col);
                    let left = (row, col - radius);

                    let border_sum = Self::down_right_sum(&down_right, top.0, top.1, right.0, right.1)
                        + Self::down_left_sum(&down_left, right.0, right.1, bottom.0, bottom.1)
                        + Self::down_right_sum(&down_right, left.0, left.1, bottom.0, bottom.1)
                        + Self::down_left_sum(&down_left, top.0, top.1, left.0, left.1)
                        - grid[top.0][top.1]
                        - grid[right.0][right.1]
                        - grid[bottom.0][bottom.1]
                        - grid[left.0][left.1];

                    Self::push_sum(&mut best, border_sum);
                }
            }
        }

        best.into_iter().rev().collect()
    }

    fn push_sum(best: &mut BTreeSet<i32>, sum: i32) {
        best.insert(sum);
        if best.len() > 3 {
            best.pop_first();
        }
    }

    fn down_right_sum(
        down_right: &[Vec<i32>],
        start_row: usize,
        start_col: usize,
        end_row: usize,
        end_col: usize,
    ) -> i32 {
        down_right[end_row + 1][end_col + 1] - down_right[start_row][start_col]
    }

    fn down_left_sum(
        down_left: &[Vec<i32>],
        start_row: usize,
        start_col: usize,
        end_row: usize,
        end_col: usize,
    ) -> i32 {
        down_left[end_row + 1][end_col] - down_left[start_row][start_col + 1]
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::get_biggest_three_rhombus_sums_in_a_grid::Solution;

    #[test]
    fn test_get_biggest_three_1() {
        let grid = [
            [3, 4, 5, 1, 3],
            [3, 3, 4, 2, 3],
            [20, 30, 200, 40, 10],
            [1, 5, 5, 4, 1],
            [4, 3, 2, 2, 5],
        ]
        .iter()
        .map(|l| l.to_vec())
        .collect::<Vec<_>>();
        assert_eq!([228, 216, 211].to_vec(), Solution::get_biggest_three(grid));
    }

    #[test]
    fn test_get_biggest_three_2() {
        let grid = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!([20, 9, 8].to_vec(), Solution::get_biggest_three(grid));
    }

    #[test]
    fn test_get_biggest_three_3() {
        let grid = [[7, 7, 7]].iter().map(|l| l.to_vec()).collect::<Vec<_>>();
        assert_eq!([7].to_vec(), Solution::get_biggest_three(grid));
    }
}
