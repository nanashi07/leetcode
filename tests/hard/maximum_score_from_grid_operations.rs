// 3225. Maximum Score From Grid Operations
// https://leetcode.com/problems/maximum-score-from-grid-operations/

struct Solution;

impl Solution {
    pub fn maximum_score(grid: Vec<Vec<i32>>) -> i64 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::maximum_score_from_grid_operations::Solution;
    use crate::shared::vec2d::to_vec2d;

    #[test]
    fn test_maximum_score_1() {
        let grid = to_vec2d([
            [0, 0, 0, 0, 0],
            [0, 0, 3, 0, 0],
            [0, 1, 0, 0, 0],
            [5, 0, 0, 3, 0],
            [0, 0, 0, 0, 2],
        ]);
        assert_eq!(11, Solution::maximum_score(grid));
    }

    #[test]
    fn test_maximum_score_2() {
        let grid = to_vec2d([
            [10, 9, 0, 0, 15],
            [7, 1, 0, 8, 0],
            [5, 20, 0, 11, 0],
            [0, 0, 0, 1, 2],
            [8, 12, 1, 10, 3],
        ]);
        assert_eq!(94, Solution::maximum_score(grid));
    }
}
