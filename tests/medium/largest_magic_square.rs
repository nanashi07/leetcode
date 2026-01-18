// 1895. Largest Magic Square
// https://leetcode.com/problems/largest-magic-square/

struct Solution;

impl Solution {
    pub fn largest_magic_square(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let max_size = m.min(n);

        // Try from the largest possible size down to 1
        for size in (1..=max_size).rev() {
            // Try all possible starting positions
            for i in 0..=m - size {
                for j in 0..=n - size {
                    if Self::is_magic_square(&grid, i, j, size) {
                        return size as i32;
                    }
                }
            }
        }

        1 // At minimum, a 1x1 square is always a magic square
    }

    fn is_magic_square(
        grid: &Vec<Vec<i32>>,
        start_row: usize,
        start_col: usize,
        size: usize,
    ) -> bool {
        if size == 1 {
            return true;
        }

        // Calculate the sum of the first row as the reference sum
        let mut target_sum = 0;
        for j in start_col..start_col + size {
            target_sum += grid[start_row][j];
        }

        // Check all rows
        for i in start_row + 1..start_row + size {
            let mut row_sum = 0;
            for j in start_col..start_col + size {
                row_sum += grid[i][j];
            }
            if row_sum != target_sum {
                return false;
            }
        }

        // Check all columns
        for j in start_col..start_col + size {
            let mut col_sum = 0;
            for i in start_row..start_row + size {
                col_sum += grid[i][j];
            }
            if col_sum != target_sum {
                return false;
            }
        }

        // Check main diagonal (top-left to bottom-right)
        let mut diag_sum = 0;
        for k in 0..size {
            diag_sum += grid[start_row + k][start_col + k];
        }
        if diag_sum != target_sum {
            return false;
        }

        // Check anti-diagonal (top-right to bottom-left)
        let mut anti_diag_sum = 0;
        for k in 0..size {
            anti_diag_sum += grid[start_row + k][start_col + size - 1 - k];
        }
        if anti_diag_sum != target_sum {
            return false;
        }

        true
    }
}
#[cfg(test)]
mod tests {
    use crate::medium::largest_magic_square::Solution;

    #[test]
    fn test_largest_magic_square_1() {
        let grid = [
            [7, 1, 4, 5, 6],
            [2, 5, 1, 6, 4],
            [1, 5, 4, 3, 2],
            [1, 2, 7, 3, 4],
        ]
        .iter()
        .map(|l| l.to_vec())
        .collect::<Vec<_>>();
        assert_eq!(3, Solution::largest_magic_square(grid));
    }

    #[test]
    fn test_largest_magic_square_2() {
        let grid = [[5, 1, 3, 1], [9, 3, 3, 1], [1, 3, 3, 8]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(2, Solution::largest_magic_square(grid));
    }
}
