// 3567. Minimum Absolute Difference in Sliding Submatrix
// https://leetcode.com/problems/minimum-absolute-difference-in-sliding-submatrix/

struct Solution;

impl Solution {
    pub fn min_abs_diff(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::minimum_absolute_difference_in_sliding_submatrix::Solution;

    #[test]
    fn test_min_abs_diff_1() {
        let grid = [[1, 8], [3, -2]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        let k = 2;
        let output = [[2]].iter().map(|l| l.to_vec()).collect::<Vec<_>>();
        assert_eq!(output, Solution::min_abs_diff(grid, k));
    }

    #[test]
    fn test_min_abs_diff_2() {
        let grid = [[3, -1]].iter().map(|l| l.to_vec()).collect::<Vec<_>>();
        let k = 1;
        let output = [[0, 0]].iter().map(|l| l.to_vec()).collect::<Vec<_>>();
        assert_eq!(output, Solution::min_abs_diff(grid, k));
    }

    #[test]
    fn test_min_abs_diff_3() {
        let grid = [[1, -2, 3], [2, 3, 5]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        let k = 2;
        let output = [[1, 2]].iter().map(|l| l.to_vec()).collect::<Vec<_>>();
        assert_eq!(output, Solution::min_abs_diff(grid, k));
    }
}
