// 3882. Minimum XOR Path in a Grid
// https://leetcode.com/problems/minimum-xor-path-in-a-grid/

struct Solution;

impl Solution {
    pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::minimum_xor_path_in_a_grid::Solution;
    use crate::shared::vec2d::to_vec2d;

    #[test]
    fn test_min_cost_1() {
        let grid = to_vec2d([[1, 2], [3, 4]]);
        assert_eq!(6, Solution::min_cost(grid));
    }

    #[test]
    fn test_min_cost_2() {
        let grid = to_vec2d([[6, 7], [5, 8]]);
        assert_eq!(9, Solution::min_cost(grid));
    }

    #[test]
    fn test_min_cost_3() {
        let grid = to_vec2d([[2, 7, 5]]);
        assert_eq!(0, Solution::min_cost(grid));
    }
}
