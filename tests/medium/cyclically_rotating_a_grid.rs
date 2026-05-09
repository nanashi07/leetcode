// 1914. Cyclically Rotating a Grid
// https://leetcode.com/problems/cyclically-rotating-a-grid/

struct Solution;

impl Solution {
    pub fn rotate_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::cyclically_rotating_a_grid::Solution;
    use crate::shared::vec2d::to_vec2d;

    #[test]
    fn test_rotate_grid_1() {
        let grid = to_vec2d([[40, 10], [30, 20]]);
        let k = 1;
        let output = to_vec2d([[10, 20], [40, 30]]);
        assert_eq!(output, Solution::rotate_grid(grid, k));
    }

    #[test]
    fn test_rotate_grid_2() {
        let grid = to_vec2d([
            [1, 2, 3, 4],
            [5, 6, 7, 8],
            [9, 10, 11, 12],
            [13, 14, 15, 16],
        ]);
        let k = 2;
        let output = to_vec2d([
            [3, 4, 8, 12],
            [2, 11, 10, 16],
            [1, 7, 6, 15],
            [5, 9, 13, 14],
        ]);
        assert_eq!(output, Solution::rotate_grid(grid, k));
    }
}
