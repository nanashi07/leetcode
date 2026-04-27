// 1391. Check if There is a Valid Path in a Grid
// https://leetcode.com/problems/check-if-there-is-a-valid-path-in-a-grid/

struct Solution;

impl Solution {
    pub fn has_valid_path(grid: Vec<Vec<i32>>) -> bool {
        todo!()
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
