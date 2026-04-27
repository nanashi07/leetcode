// 1391. Check if There is a Valid Path in a Grid
// https://leetcode.com/problems/check-if-there-is-a-valid-path-in-a-grid/

struct Solution;

impl Solution {
    pub fn has_valid_path(grid: Vec<Vec<i32>>) -> bool {
        let m = grid.len();
        let n = grid[0].len();
        if m == 1 && n == 1 {
            return true;
        }

        // Directions: 0=left, 1=right, 2=up, 3=down
        // Each street opens to certain directions
        let openings: [&[usize]; 7] = [
            &[],
            &[0, 1], // street 1: left, right
            &[2, 3], // street 2: up, down
            &[0, 3], // street 3: left, down
            &[1, 3], // street 4: right, down
            &[0, 2], // street 5: left, up
            &[1, 2], // street 6: right, up
        ];

        // Direction deltas: left, right, up, down
        let dr: [i32; 4] = [0, 0, -1, 1];
        let dc: [i32; 4] = [-1, 1, 0, 0];
        // Opposite direction
        let opposite = [1, 0, 3, 2];

        let mut visited = vec![vec![false; n]; m];
        let mut queue = std::collections::VecDeque::new();
        queue.push_back((0, 0));
        visited[0][0] = true;

        while let Some((r, c)) = queue.pop_front() {
            let street = grid[r][c] as usize;
            for &dir in openings[street] {
                let nr = r as i32 + dr[dir];
                let nc = c as i32 + dc[dir];
                if nr < 0 || nr >= m as i32 || nc < 0 || nc >= n as i32 {
                    continue;
                }
                let (nr, nc) = (nr as usize, nc as usize);
                if visited[nr][nc] {
                    continue;
                }
                let neighbor = grid[nr][nc] as usize;
                if openings[neighbor].contains(&opposite[dir]) {
                    if nr == m - 1 && nc == n - 1 {
                        return true;
                    }
                    visited[nr][nc] = true;
                    queue.push_back((nr, nc));
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::check_if_there_is_a_valid_path_in_a_grid::Solution;
    use crate::shared::vec2d::to_vec2d;

    #[test]
    fn test_has_valid_path_1() {
        let grid = to_vec2d([[2, 4, 3], [6, 5, 2]]);
        assert_eq!(true, Solution::has_valid_path(grid));
    }

    #[test]
    fn test_has_valid_path_2() {
        let grid = to_vec2d([[1, 2, 1], [1, 2, 1]]);
        assert_eq!(false, Solution::has_valid_path(grid));
    }

    #[test]
    fn test_has_valid_path_3() {
        let grid = to_vec2d([[1, 1, 2]]);
        assert_eq!(false, Solution::has_valid_path(grid));
    }
}
