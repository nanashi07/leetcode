// 2257. Count Unguarded Cells in the Grid
// https://leetcode.com/problems/count-unguarded-cells-in-the-grid/

struct Solution;

impl Solution {
    pub fn count_unguarded(m: i32, n: i32, guards: Vec<Vec<i32>>, walls: Vec<Vec<i32>>) -> i32 {
        let m = m as usize;
        let n = n as usize;

        // 0 = unguarded, 1 = guard, 2 = wall, 3 = guarded
        let mut grid = vec![vec![0; n]; m];

        // Place guards
        for guard in &guards {
            let (r, c) = (guard[0] as usize, guard[1] as usize);
            grid[r][c] = 1;
        }

        // Place walls
        for wall in &walls {
            let (r, c) = (wall[0] as usize, wall[1] as usize);
            grid[r][c] = 2;
        }

        // For each guard, mark all cells they can see
        for guard in &guards {
            let (r, c) = (guard[0] as usize, guard[1] as usize);

            // Look up
            for i in (0..r).rev() {
                if grid[i][c] == 1 || grid[i][c] == 2 {
                    break; // Hit guard or wall
                }
                grid[i][c] = 3; // Mark as guarded
            }

            // Look down
            for i in r + 1..m {
                if grid[i][c] == 1 || grid[i][c] == 2 {
                    break; // Hit guard or wall
                }
                grid[i][c] = 3; // Mark as guarded
            }

            // Look left
            for j in (0..c).rev() {
                if grid[r][j] == 1 || grid[r][j] == 2 {
                    break; // Hit guard or wall
                }
                grid[r][j] = 3; // Mark as guarded
            }

            // Look right
            for j in c + 1..n {
                if grid[r][j] == 1 || grid[r][j] == 2 {
                    break; // Hit guard or wall
                }
                grid[r][j] = 3; // Mark as guarded
            }
        }

        // Count unguarded cells (cells with value 0)
        let mut count = 0;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 0 {
                    count += 1;
                }
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::count_unguarded_cells_in_the_grid::Solution;

    #[test]
    fn test_count_unguarded_1() {
        let m = 4;
        let n = 6;
        let guards = [[0, 0], [1, 1], [2, 3]]
            .into_iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        let walls = [[0, 1], [2, 2], [1, 4]]
            .into_iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(7, Solution::count_unguarded(m, n, guards, walls));
    }

    #[test]
    fn test_count_unguarded_2() {
        let m = 3;
        let n = 3;
        let guards = [[1, 1]].into_iter().map(|l| l.to_vec()).collect::<Vec<_>>();
        let walls = [[0, 1], [1, 0], [2, 1], [1, 2]]
            .into_iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(4, Solution::count_unguarded(m, n, guards, walls));
    }
}
