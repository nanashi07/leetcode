// 3548. Equal Sum Grid Partition II
// https://leetcode.com/problems/equal-sum-grid-partition-ii/

use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn can_partition_grid(grid: Vec<Vec<i32>>) -> bool {
        let m = grid.len();
        let n = grid[0].len();
        let total: i64 = grid.iter().flat_map(|r| r.iter()).map(|&v| v as i64).sum();

        let trans = Self::transpose(&grid, m, n);
        Self::check(&grid, total)
            || Self::check(&Self::rev(&grid), total)
            || Self::check(&Self::rev(&trans), total)
            || Self::check(&trans, total)
    }

    fn check(g: &[Vec<i32>], total: i64) -> bool {
        let m = g.len();
        if m < 2 {
            return false;
        }
        let n = g[0].len();
        let mut top: i64 = 0;
        let mut seen = HashSet::new();
        for i in 0..m - 1 {
            top += g[i].iter().map(|&v| v as i64).sum::<i64>();
            let diff = 2 * top - total; // topSum - botSum
            for &v in &g[i] {
                seen.insert(v as i64);
            }
            if diff == 0
                || diff == g[0][0] as i64
                || diff == g[0][n - 1] as i64
                || diff == g[i][0] as i64
            {
                return true;
            }
            if n > 1 && i > 0 && seen.contains(&diff) {
                return true;
            }
        }
        false
    }

    fn rev(g: &[Vec<i32>]) -> Vec<Vec<i32>> {
        g.iter().rev().cloned().collect()
    }

    fn transpose(g: &[Vec<i32>], m: usize, n: usize) -> Vec<Vec<i32>> {
        (0..n)
            .map(|j| (0..m).map(|i| g[i][j]).collect())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::equal_sum_grid_partition_ii::Solution;
    use crate::shared::vec2d::to_vec2d;

    #[test]
    fn test_can_partition_grid_1() {
        let grid = to_vec2d([[1, 4], [2, 3]]);
        assert_eq!(true, Solution::can_partition_grid(grid));
    }

    #[test]
    fn test_can_partition_grid_2() {
        let grid = to_vec2d([[1, 2], [3, 4]]);
        assert_eq!(true, Solution::can_partition_grid(grid));
    }

    #[test]
    fn test_can_partition_grid_3() {
        let grid = to_vec2d([[1, 2, 4], [2, 3, 5]]);
        assert_eq!(false, Solution::can_partition_grid(grid));
    }

    #[test]
    fn test_can_partition_grid_4() {
        let grid = to_vec2d([[4, 1, 8], [3, 2, 6]]);
        assert_eq!(false, Solution::can_partition_grid(grid));
    }
}
