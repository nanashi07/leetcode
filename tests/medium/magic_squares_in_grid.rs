// 840. Magic Squares In Grid
// https://leetcode.com/problems/magic-squares-in-grid/

struct Solution;

impl Solution {
    pub fn num_magic_squares_inside(grid: Vec<Vec<i32>>) -> i32 {
        let rows = grid.len();
        let cols = grid[0].len();

        // Need at least 3x3 grid
        if rows < 3 || cols < 3 {
            return 0;
        }

        let mut count = 0;

        // Check each possible 3x3 subgrid
        for i in 0..=rows - 3 {
            for j in 0..=cols - 3 {
                if Self::is_magic_square(&grid, i, j) {
                    count += 1;
                }
            }
        }

        count
    }

    fn is_magic_square(grid: &Vec<Vec<i32>>, row: usize, col: usize) -> bool {
        // Check if all numbers are between 1-9 and distinct
        let mut seen = [false; 10];

        for i in 0..3 {
            for j in 0..3 {
                let num = grid[row + i][col + j];
                if num < 1 || num > 9 {
                    return false;
                }
                if seen[num as usize] {
                    return false;
                }
                seen[num as usize] = true;
            }
        }

        // Check if all rows sum to 15
        for i in 0..3 {
            let sum = grid[row + i][col] + grid[row + i][col + 1] + grid[row + i][col + 2];
            if sum != 15 {
                return false;
            }
        }

        // Check if all columns sum to 15
        for j in 0..3 {
            let sum = grid[row][col + j] + grid[row + 1][col + j] + grid[row + 2][col + j];
            if sum != 15 {
                return false;
            }
        }

        // Check diagonal (top-left to bottom-right)
        let diag1 = grid[row][col] + grid[row + 1][col + 1] + grid[row + 2][col + 2];
        if diag1 != 15 {
            return false;
        }

        // Check diagonal (top-right to bottom-left)
        let diag2 = grid[row][col + 2] + grid[row + 1][col + 1] + grid[row + 2][col];
        if diag2 != 15 {
            return false;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::magic_squares_in_grid::Solution;

    #[test]
    fn test_num_magic_squares_inside_1() {
        let grid = [[4, 3, 8, 4], [9, 5, 1, 9], [2, 7, 6, 2]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(1, Solution::num_magic_squares_inside(grid));
    }

    #[test]
    fn test_num_magic_squares_inside_2() {
        let grid = [[8]].iter().map(|l| l.to_vec()).collect::<Vec<_>>();
        assert_eq!(0, Solution::num_magic_squares_inside(grid));
    }
}
