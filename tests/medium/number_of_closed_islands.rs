// 1254. Number of Closed Islands
// https://leetcode.com/problems/number-of-closed-islands/

struct Solution;

impl Solution {
    pub fn closed_island(grid: Vec<Vec<i32>>) -> i32 {
        todo!()
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
