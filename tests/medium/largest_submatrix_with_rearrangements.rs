// 1727. Largest Submatrix With Rearrangements
// https://leetcode.com/problems/largest-submatrix-with-rearrangements/

struct Solution;

impl Solution {
    pub fn largest_submatrix(matrix: Vec<Vec<i32>>) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut heights = vec![0i32; n];
        let mut ans = 0;

        for i in 0..m {
            // Update column heights
            for j in 0..n {
                heights[j] = if matrix[i][j] == 1 { heights[j] + 1 } else { 0 };
            }
            // Sort descending; for each width k+1, area = heights[k] * (k+1)
            let mut sorted = heights.clone();
            sorted.sort_unstable_by(|a, b| b.cmp(a));
            for (k, &h) in sorted.iter().enumerate() {
                if h == 0 { break; }
                ans = ans.max(h * (k as i32 + 1));
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::largest_submatrix_with_rearrangements::Solution;
    use crate::shared::vec2d::to_vec2d;

    #[test]
    fn test_largest_submatrix_1() {
        let matrix = to_vec2d([[0, 0, 1], [1, 1, 1], [1, 0, 1]]);
        assert_eq!(4, Solution::largest_submatrix(matrix));
    }

    #[test]
    fn test_largest_submatrix_2() {
        let matrix = to_vec2d([[1, 0, 1, 0, 1]]);
        assert_eq!(3, Solution::largest_submatrix(matrix));
    }

    #[test]
    fn test_largest_submatrix_3() {
        let matrix = to_vec2d([[1, 1, 0], [1, 0, 1]]);
        assert_eq!(2, Solution::largest_submatrix(matrix));
    }
}
