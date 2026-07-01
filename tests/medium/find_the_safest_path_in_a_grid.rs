// 2812. Find the Safest Path in a Grid
// https://leetcode.com/problems/find-the-safest-path-in-a-grid/

struct Solution;

impl Solution {
    pub fn maximum_safeness_factor(grid: Vec<Vec<i32>>) -> i32 {
        todo!()
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
