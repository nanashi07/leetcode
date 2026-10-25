// 1254. Number of Closed Islands
// https://leetcode.com/problems/number-of-closed-islands/

struct Solution;

const DIRS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

impl Solution {
    /// O(rows * cols) time, O(rows * cols) worst-case stack space.
    ///
    /// Flood fill every land (`0`) region. A region is a closed island only if it
    /// never reaches the grid border, so we start the search from inner cells and
    /// flag the region as open as soon as any neighbour falls outside the grid.
    pub fn closed_island(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let rows = grid.len();
        if rows < 3 {
            return 0;
        }
        let cols = grid[0].len();
        if cols < 3 {
            return 0;
        }

        let mut count = 0;
        for r in 1..rows - 1 {
            for c in 1..cols - 1 {
                if grid[r][c] != 0 {
                    continue;
                }

                // Flood fill this island, sinking it as we go (water == 1).
                let mut closed = true;
                let mut stack = vec![(r, c)];
                grid[r][c] = 1;
                while let Some((y, x)) = stack.pop() {
                    for (dy, dx) in DIRS {
                        let ny = y as i32 + dy;
                        let nx = x as i32 + dx;
                        if ny < 0 || nx < 0 || ny as usize >= rows || nx as usize >= cols {
                            closed = false;
                            continue;
                        }
                        let (ny, nx) = (ny as usize, nx as usize);
                        if grid[ny][nx] == 0 {
                            grid[ny][nx] = 1;
                            stack.push((ny, nx));
                        }
                    }
                }

                count += closed as i32;
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::number_of_closed_islands::Solution;
    use crate::shared::vec2d::to_vec2d;

    #[test]
    fn test_closed_island_1() {
        let grid = to_vec2d([
            [1, 1, 1, 1, 1, 1, 1, 0],
            [1, 0, 0, 0, 0, 1, 1, 0],
            [1, 0, 1, 0, 1, 1, 1, 0],
            [1, 0, 0, 0, 0, 1, 0, 1],
            [1, 1, 1, 1, 1, 1, 1, 0],
        ]);
        assert_eq!(2, Solution::closed_island(grid));
    }

    #[test]
    fn test_closed_island_2() {
        let grid = to_vec2d([[0, 0, 1, 0, 0], [0, 1, 0, 1, 0], [0, 1, 1, 1, 0]]);
        assert_eq!(1, Solution::closed_island(grid));
    }
}
