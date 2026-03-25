// 3546. Equal Sum Grid Partition I
// https://leetcode.com/problems/equal-sum-grid-partition-i/

struct Solution;

impl Solution {
    pub fn can_partition_grid(grid: Vec<Vec<i32>>) -> bool {
        let total: i64 = grid.iter().flatten().map(|&x| x as i64).sum();

        // Check horizontal cuts (between rows)
        let mut prefix: i64 = 0;
        for row in &grid[..grid.len() - 1] {
            prefix += row.iter().map(|&x| x as i64).sum::<i64>();
            if prefix * 2 == total {
                return true;
            }
        }

        // Check vertical cuts (between columns)
        let cols = grid[0].len();
        let col_sums: Vec<i64> = (0..cols)
            .map(|c| grid.iter().map(|row| row[c] as i64).sum())
            .collect();
        let mut prefix: i64 = 0;
        for &cs in &col_sums[..cols - 1] {
            prefix += cs;
            if prefix * 2 == total {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::equal_sum_grid_partition_i::Solution;
    use crate::shared::vec2d::to_vec2d;

    #[test]
    fn test_can_partition_grid_1() {
        let grid = to_vec2d([[1, 4], [2, 3]]);
        assert_eq!(true, Solution::can_partition_grid(grid));
    }
}
