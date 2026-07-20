// 1260. Shift 2D Grid
// https://leetcode.com/problems/shift-2d-grid/

struct Solution;

impl Solution {
    pub fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::shift_2d_grid::Solution;
    use crate::shared::vec2d::to_vec2d;

    #[test]
    fn test_shift_grid_1() {
        let grid = to_vec2d([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
        let k = 1;
        let output = to_vec2d([[9, 1, 2], [3, 4, 5], [6, 7, 8]]);
        assert_eq!(output, Solution::shift_grid(grid, k));
    }

    #[test]
    fn test_shift_grid_2() {
        let grid = to_vec2d([[3, 8, 1, 9], [19, 7, 2, 5], [4, 6, 11, 10], [12, 0, 21, 13]]);
        let k = 4;
        let output = to_vec2d([[12, 0, 21, 13], [3, 8, 1, 9], [19, 7, 2, 5], [4, 6, 11, 10]]);
        assert_eq!(output, Solution::shift_grid(grid, k));
    }

    #[test]
    fn test_shift_grid_3() {
        let grid = to_vec2d([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
        let k = 9;
        let output = to_vec2d([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
        assert_eq!(output, Solution::shift_grid(grid, k));
    }
}
