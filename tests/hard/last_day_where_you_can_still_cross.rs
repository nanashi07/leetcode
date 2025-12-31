// 1970. Last Day Where You Can Still Cross
// https://leetcode.com/problems/last-day-where-you-can-still-cross/

struct Solution;

impl Solution {
    pub fn latest_day_to_cross(row: i32, col: i32, cells: Vec<Vec<i32>>) -> i32 {
        let row = row as usize;
        let col = col as usize;

        // Binary search on the answer
        let mut left = 1;
        let mut right = cells.len();
        let mut result = 0;

        while left <= right {
            let mid = left + (right - left) / 2;

            if Self::can_cross(row, col, &cells, mid) {
                result = mid;
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        result as i32
    }

    fn can_cross(row: usize, col: usize, cells: &Vec<Vec<i32>>, day: usize) -> bool {
        // Create grid with water cells marked up to 'day'
        let mut grid = vec![vec![false; col]; row];

        // Mark cells as water for first 'day' days
        for i in 0..day {
            let r = cells[i][0] as usize - 1;
            let c = cells[i][1] as usize - 1;
            grid[r][c] = true; // true means water
        }

        // BFS from top row to bottom row
        use std::collections::VecDeque;
        let mut queue = VecDeque::new();
        let mut visited = vec![vec![false; col]; row];

        // Start from all cells in the first row that are land
        for c in 0..col {
            if !grid[0][c] {
                queue.push_back((0, c));
                visited[0][c] = true;
            }
        }

        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        while let Some((r, c)) = queue.pop_front() {
            // If we reached the last row, we can cross
            if r == row - 1 {
                return true;
            }

            for &(dr, dc) in &directions {
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;

                if nr >= 0 && nr < row as i32 && nc >= 0 && nc < col as i32 {
                    let nr = nr as usize;
                    let nc = nc as usize;

                    if !grid[nr][nc] && !visited[nr][nc] {
                        visited[nr][nc] = true;
                        queue.push_back((nr, nc));
                    }
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::last_day_where_you_can_still_cross::Solution;

    #[test]
    fn test_latest_day_to_cross_1() {
        let row = 2;
        let col = 2;
        let cells = [[1, 1], [2, 1], [1, 2], [2, 2]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(2, Solution::latest_day_to_cross(row, col, cells));
    }

    #[test]
    fn test_latest_day_to_cross_2() {
        let row = 2;
        let col = 2;
        let cells = [[1, 1], [1, 2], [2, 1], [2, 2]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(1, Solution::latest_day_to_cross(row, col, cells));
    }

    #[test]
    fn test_latest_day_to_cross_3() {
        let row = 3;
        let col = 3;
        let cells = [
            [1, 2],
            [2, 1],
            [3, 3],
            [2, 2],
            [1, 1],
            [1, 3],
            [2, 3],
            [3, 2],
            [3, 1],
        ]
        .iter()
        .map(|l| l.to_vec())
        .collect::<Vec<_>>();
        assert_eq!(3, Solution::latest_day_to_cross(row, col, cells));
    }
}
