// # 778. Swim in Rising Water
// https://leetcode.com/problems/swim-in-rising-water/

use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut visited = vec![vec![false; n]; n];

        // Min-heap: (max_elevation_so_far, row, col)
        let mut heap = BinaryHeap::new();
        heap.push(Reverse((grid[0][0], 0, 0)));

        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        while let Some(Reverse((max_elev, r, c))) = heap.pop() {
            // If already visited, skip
            if visited[r][c] {
                continue;
            }

            visited[r][c] = true;

            // Reached destination
            if r == n - 1 && c == n - 1 {
                return max_elev;
            }

            // Explore neighbors
            for (dr, dc) in directions.iter() {
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;

                // Check bounds
                if nr >= 0 && nr < n as i32 && nc >= 0 && nc < n as i32 {
                    let nr = nr as usize;
                    let nc = nc as usize;

                    if !visited[nr][nc] {
                        // The time needed is the max of current path and new cell
                        let new_max_elev = max_elev.max(grid[nr][nc]);
                        heap.push(Reverse((new_max_elev, nr, nc)));
                    }
                }
            }
        }

        -1 // Should never reach here with valid input
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::swim_in_rising_water::Solution;

    #[test]
    fn test_swim_in_water_1() {
        let grid = [[0, 2], [1, 3]]
            .into_iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(3, Solution::swim_in_water(grid));
    }

    #[test]
    fn test_swim_in_water_2() {
        let grid = [
            [0, 1, 2, 3, 4],
            [24, 23, 22, 21, 5],
            [12, 13, 14, 15, 16],
            [11, 17, 18, 19, 20],
            [10, 9, 8, 7, 6],
        ]
        .into_iter()
        .map(|l| l.to_vec())
        .collect::<Vec<_>>();
        assert_eq!(16, Solution::swim_in_water(grid));
    }

    #[test]
    fn test_swim_in_water_3() {
        let grid = [[0, 3], [1, 2]]
            .into_iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(2, Solution::swim_in_water(grid));
    }

    #[test]
    fn test_swim_in_water_4() {
        let grid = [[10, 12, 4, 6], [9, 11, 3, 5], [1, 7, 13, 8], [2, 0, 15, 14]]
            .into_iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(14, Solution::swim_in_water(grid));
    }
}
