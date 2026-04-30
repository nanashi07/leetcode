// 3742. Maximum Path Score in a Grid
// https://leetcode.com/problems/maximum-path-score-in-a-grid/

struct Solution;

impl Solution {
    pub fn max_path_score(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::maximum_path_score_in_a_grid::Solution;
    use crate::shared::vec2d::to_vec2d;

    #[test]
    fn test_max_path_score_1() {
        let grid = to_vec2d([[0, 1], [2, 0]]);
        let k = 1;
        assert_eq!(2, Solution::max_path_score(grid, k));
    }

    #[test]
    fn test_max_path_score_2() {
        let grid = to_vec2d([[0, 1], [1, 2]]);
        let k = 1;
        assert_eq!(-1, Solution::max_path_score(grid, k));
    }
}
