// 3286. Find a Safe Walk Through a Grid
// https://leetcode.com/problems/find-a-safe-walk-through-a-grid/

use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    pub fn find_safe_walk(grid: Vec<Vec<i32>>, health: i32) -> bool {
        let rows = grid.len();
        let cols = grid[0].len();
        let mut dist = vec![vec![i32::MAX; cols]; rows];
        let mut heap = BinaryHeap::new();

        dist[0][0] = grid[0][0];
        heap.push((Reverse(dist[0][0]), 0usize, 0usize));

        let dirs = [(-1isize, 0isize), (1, 0), (0, -1), (0, 1)];

        while let Some((Reverse(cost), row, col)) = heap.pop() {
            if cost != dist[row][col] {
                continue;
            }

            if row == rows - 1 && col == cols - 1 {
                return cost < health;
            }

            for (dr, dc) in dirs {
                let next_row = row as isize + dr;
                let next_col = col as isize + dc;

                if next_row < 0
                    || next_row >= rows as isize
                    || next_col < 0
                    || next_col >= cols as isize
                {
                    continue;
                }

                let next_row = next_row as usize;
                let next_col = next_col as usize;
                let next_cost = cost + grid[next_row][next_col];

                if next_cost < dist[next_row][next_col] {
                    dist[next_row][next_col] = next_cost;
                    heap.push((Reverse(next_cost), next_row, next_col));
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::find_a_safe_walk_through_a_grid::Solution;
    use crate::shared::vec2d::to_vec2d;

    #[test]
    fn test_find_safe_walk_1() {
        let grid = to_vec2d([[0, 1, 0, 0, 0], [0, 1, 0, 1, 0], [0, 0, 0, 1, 0]]);
        let health = 1;
        assert!(Solution::find_safe_walk(grid, health));
    }

    #[test]
    fn test_find_safe_walk_2() {
        let grid = to_vec2d([
            [0, 1, 1, 0, 0, 0],
            [1, 0, 1, 0, 0, 0],
            [0, 1, 1, 1, 0, 1],
            [0, 0, 1, 0, 1, 0],
        ]);
        let health = 3;
        assert!(!Solution::find_safe_walk(grid, health));
    }

    #[test]
    fn test_find_safe_walk_3() {
        let grid = to_vec2d([[1, 1, 1], [1, 0, 1], [1, 1, 1]]);
        let health = 5;
        assert!(Solution::find_safe_walk(grid, health));
    }
}
