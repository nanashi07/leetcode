// 2812. Find the Safest Path in a Grid
// https://leetcode.com/problems/find-the-safest-path-in-a-grid/

struct Solution;

impl Solution {
    pub fn maximum_safeness_factor(grid: Vec<Vec<i32>>) -> i32 {
        use std::collections::VecDeque;

        let n = grid.len();
        let mut dist = vec![vec![i32::MAX; n]; n];
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();

        for i in 0..n {
            for j in 0..n {
                if grid[i][j] == 1 {
                    dist[i][j] = 0;
                    queue.push_back((i, j));
                }
            }
        }

        let dirs: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        while let Some((r, c)) = queue.pop_front() {
            for (dr, dc) in dirs {
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;
                if nr >= 0 && nr < n as i32 && nc >= 0 && nc < n as i32 {
                    let (nr, nc) = (nr as usize, nc as usize);
                    if dist[nr][nc] == i32::MAX {
                        dist[nr][nc] = dist[r][c] + 1;
                        queue.push_back((nr, nc));
                    }
                }
            }
        }

        if dist[0][0] == 0 || dist[n - 1][n - 1] == 0 {
            return 0;
        }

        let can_reach = |min_safe: i32| -> bool {
            if dist[0][0] < min_safe || dist[n - 1][n - 1] < min_safe {
                return false;
            }
            let mut visited = vec![vec![false; n]; n];
            let mut q: VecDeque<(usize, usize)> = VecDeque::new();
            visited[0][0] = true;
            q.push_back((0, 0));
            while let Some((r, c)) = q.pop_front() {
                if r == n - 1 && c == n - 1 {
                    return true;
                }
                for (dr, dc) in dirs {
                    let nr = r as i32 + dr;
                    let nc = c as i32 + dc;
                    if nr >= 0 && nr < n as i32 && nc >= 0 && nc < n as i32 {
                        let (nr, nc) = (nr as usize, nc as usize);
                        if !visited[nr][nc] && dist[nr][nc] >= min_safe {
                            visited[nr][nc] = true;
                            q.push_back((nr, nc));
                        }
                    }
                }
            }
            false
        };

        let mut lo = 0i32;
        let mut hi = n as i32;
        while lo < hi {
            let mid = (lo + hi + 1) / 2;
            if can_reach(mid) {
                lo = mid;
            } else {
                hi = mid - 1;
            }
        }
        lo
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::find_the_safest_path_in_a_grid::Solution;
    use crate::shared::vec2d::to_vec2d;

    #[test]
    fn test_maximum_safeness_factor_1() {
        let grid = to_vec2d([[1, 0, 0], [0, 0, 0], [0, 0, 1]]);
        assert_eq!(0, Solution::maximum_safeness_factor(grid));
    }

    #[test]
    fn test_maximum_safeness_factor_2() {
        let grid = to_vec2d([[0, 0, 1], [0, 0, 0], [0, 0, 0]]);
        assert_eq!(2, Solution::maximum_safeness_factor(grid));
    }

    #[test]
    fn test_maximum_safeness_factor_3() {
        let grid = to_vec2d([[0, 0, 0, 1], [0, 0, 0, 0], [0, 0, 0, 0], [1, 0, 0, 0]]);
        assert_eq!(3, Solution::maximum_safeness_factor(grid));
    }
}
