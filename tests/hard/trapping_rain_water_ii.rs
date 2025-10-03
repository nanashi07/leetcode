// # 407. Trapping Rain Water II
// https://leetcode.com/problems/trapping-rain-water-ii/

use std::collections::BinaryHeap;

struct Solution;

#[derive(Eq, PartialEq)]
struct Cell {
    height: i32,
    row: usize,
    col: usize,
}

impl Ord for Cell {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.height.cmp(&self.height) // Min-heap
    }
}

impl PartialOrd for Cell {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
        if height_map.is_empty() || height_map[0].is_empty() {
            return 0;
        }

        let m = height_map.len();
        let n = height_map[0].len();

        if m < 3 || n < 3 {
            return 0; // No interior cells to trap water
        }

        let mut visited = vec![vec![false; n]; m];
        let mut heap = BinaryHeap::new();

        // Add all boundary cells to heap
        for i in 0..m {
            for j in 0..n {
                if i == 0 || i == m - 1 || j == 0 || j == n - 1 {
                    heap.push(Cell {
                        height: height_map[i][j],
                        row: i,
                        col: j,
                    });
                    visited[i][j] = true;
                }
            }
        }

        let mut water = 0;
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        // Process cells from lowest to highest boundary
        while let Some(cell) = heap.pop() {
            // Check all four neighbors
            for (dr, dc) in &directions {
                let new_row = cell.row as i32 + dr;
                let new_col = cell.col as i32 + dc;

                if new_row < 0 || new_row >= m as i32 || new_col < 0 || new_col >= n as i32 {
                    continue;
                }

                let new_row = new_row as usize;
                let new_col = new_col as usize;

                if visited[new_row][new_col] {
                    continue;
                }

                visited[new_row][new_col] = true;

                // Water level at this cell is at least as high as current cell
                // If the neighbor is lower, it can trap water
                let neighbor_height = height_map[new_row][new_col];
                if neighbor_height < cell.height {
                    water += cell.height - neighbor_height;
                }

                // Push neighbor with max height (acts as new boundary)
                heap.push(Cell {
                    height: cell.height.max(neighbor_height),
                    row: new_row,
                    col: new_col,
                });
            }
        }

        water
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::trapping_rain_water_ii::Solution;

    #[test]
    fn test_trap_rain_water_1() {
        let height_map = [[1, 4, 3, 1, 3, 2], [3, 2, 1, 3, 2, 4], [2, 3, 3, 2, 3, 1]]
            .into_iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(4, Solution::trap_rain_water(height_map));
    }

    #[test]
    fn test_trap_rain_water_2() {
        let height_map = [
            [3, 3, 3, 3, 3],
            [3, 2, 2, 2, 3],
            [3, 2, 1, 2, 3],
            [3, 2, 2, 2, 3],
            [3, 3, 3, 3, 3],
        ]
        .into_iter()
        .map(|l| l.to_vec())
        .collect::<Vec<_>>();
        assert_eq!(10, Solution::trap_rain_water(height_map));
    }
}
