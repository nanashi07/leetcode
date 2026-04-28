// 2033. Minimum Operations to Make a Uni-Value Grid
// https://leetcode.com/problems/minimum-operations-to-make-a-uni-value-grid/

struct Solution;

impl Solution {
    pub fn min_operations(grid: Vec<Vec<i32>>, x: i32) -> i32 {
        let mut vals: Vec<i32> = grid.into_iter().flatten().collect();
        let r = vals[0] % x;
        if vals.iter().any(|&v| v % x != r) {
            return -1;
        }
        vals.sort_unstable();
        let median = vals[vals.len() / 2];
        vals.iter().map(|&v| (v - median).abs() / x).sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::minimum_operations_to_make_a_uni_value_grid::Solution;
    use crate::shared::vec2d::to_vec2d;

    #[test]
    fn test_min_operations_1() {
        let grid = to_vec2d([[2, 4], [6, 8]]);
        let x = 2;
        assert_eq!(4, Solution::min_operations(grid, x));
    }

    #[test]
    fn test_min_operations_2() {
        let grid = to_vec2d([[1, 5], [2, 3]]);
        let x = 1;
        assert_eq!(5, Solution::min_operations(grid, x));
    }

    #[test]
    fn test_min_operations_3() {
        let grid = to_vec2d([[1, 2], [3, 4]]);
        let x = 2;
        assert_eq!(-1, Solution::min_operations(grid, x));
    }
}
