// 3286. Find a Safe Walk Through a Grid
// https://leetcode.com/problems/find-a-safe-walk-through-a-grid/

struct Solution;

impl Solution {
    pub fn find_safe_walk(grid: Vec<Vec<i32>>, health: i32) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::find_a_safe_walk_through_a_grid::Solution;
    use crate::shared::vec2d::to_vec2d;

    #[test]
    fn test_find_safe_walk_1() {
        let grid = to_vec2d([[0, 1, 0, 0, 0], [0, 1, 0, 1, 0], [0, 0, 0, 1, 0]]);
        let health = 1;
        assert_eq!(true, Solution::find_safe_walk(grid, health));
    }

    #[test]
    fn test_find_safe_walk_2() {
        let grid = to_vec2d([
            [0, 1, 1, 0, 0, 0],
            [1, 0, 1, 0, 0, 0],
            [0, 1, 1, 1, 0, 1],
            [0, 0, 1, 0, 1, 0],
        ]);
        let health = 3;
        assert_eq!(false, Solution::find_safe_walk(grid, health));
    }

    #[test]
    fn test_find_safe_walk_3() {
        let grid = to_vec2d([[1, 1, 1], [1, 0, 1], [1, 1, 1]]);
        let health = 5;
        assert_eq!(true, Solution::find_safe_walk(grid, health));
    }
}
