// 3567. Minimum Absolute Difference in Sliding Submatrix
// https://leetcode.com/problems/minimum-absolute-difference-in-sliding-submatrix/

use std::collections::BTreeMap;

struct Solution;

impl Solution {
    pub fn min_abs_diff(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let k = k as usize;
        let m = grid.len();
        let n = grid[0].len();
        let rows = m - k + 1;
        let cols = n - k + 1;
        let mut result = vec![vec![0i32; cols]; rows];

        if k == 1 {
            return result;
        }

        for i in 0..rows {
            // Initialize BTreeMap for the first window in this row band
            let mut counts: BTreeMap<i32, i32> = BTreeMap::new();
            for r in i..i + k {
                for c in 0..k {
                    *counts.entry(grid[r][c]).or_insert(0) += 1;
                }
            }

            result[i][0] = Self::min_diff_of_distinct(&counts);

            for j in 1..cols {
                // Slide right: remove outgoing column (j-1), add incoming column (j+k-1)
                for r in i..i + k {
                    let out_val = grid[r][j - 1];
                    let cnt = counts.get_mut(&out_val).unwrap();
                    if *cnt == 1 {
                        counts.remove(&out_val);
                    } else {
                        *cnt -= 1;
                    }
                    *counts.entry(grid[r][j + k - 1]).or_insert(0) += 1;
                }
                result[i][j] = Self::min_diff_of_distinct(&counts);
            }
        }

        result
    }

    /// Finds the minimum absolute difference between any two distinct keys in the BTreeMap.
    /// BTreeMap keys are in sorted order, so we only need to check consecutive keys.
    fn min_diff_of_distinct(counts: &BTreeMap<i32, i32>) -> i32 {
        let mut prev = None;
        let mut min_diff = i32::MAX;
        for &key in counts.keys() {
            if let Some(p) = prev {
                let diff = key - p;
                if diff < min_diff {
                    min_diff = diff;
                    if min_diff == 1 {
                        break; // Minimum possible gap for integers
                    }
                }
            }
            prev = Some(key);
        }
        if min_diff == i32::MAX { 0 } else { min_diff }
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::minimum_absolute_difference_in_sliding_submatrix::Solution;
    use crate::shared::vec2d::to_vec2d;

    #[test]
    fn test_min_abs_diff_1() {
        let grid = to_vec2d([[1, 8], [3, -2]]);
        let k = 2;
        let output = to_vec2d([[2]]);
        assert_eq!(output, Solution::min_abs_diff(grid, k));
    }

    #[test]
    fn test_min_abs_diff_2() {
        let grid = to_vec2d([[3, -1]]);
        let k = 1;
        let output = to_vec2d([[0, 0]]);
        assert_eq!(output, Solution::min_abs_diff(grid, k));
    }

    #[test]
    fn test_min_abs_diff_3() {
        let grid = to_vec2d([[1, -2, 3], [2, 3, 5]]);
        let k = 2;
        let output = to_vec2d([[1, 2]]);
        assert_eq!(output, Solution::min_abs_diff(grid, k));
    }
}
