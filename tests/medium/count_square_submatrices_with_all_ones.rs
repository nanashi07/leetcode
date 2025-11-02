// 1277. Count Square Submatrices with All Ones
// https://leetcode.com/problems/count-square-submatrices-with-all-ones/

struct Solution;

impl Solution {
    pub fn count_squares(matrix: Vec<Vec<i32>>) -> i32 {
        if matrix.is_empty() || matrix[0].is_empty() {
            return 0;
        }

        let rows = matrix.len();
        let cols = matrix[0].len();
        let mut dp = matrix; // Use the input matrix as DP table
        let mut count = 0;

        // Process each cell
        for i in 0..rows {
            for j in 0..cols {
                if dp[i][j] == 1 {
                    if i > 0 && j > 0 {
                        // For cells not on the first row/column:
                        // dp[i][j] = min of three neighbors + 1
                        dp[i][j] = 1 + dp[i - 1][j].min(dp[i][j - 1]).min(dp[i - 1][j - 1]);
                    }
                    // dp[i][j] now represents the side length of the largest square
                    // ending at position (i,j), which equals the number of squares ending here
                    count += dp[i][j];
                }
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::count_square_submatrices_with_all_ones::Solution;

    #[test]
    fn test_count_squares_1() {
        let matrix = [[0, 1, 1, 1], [1, 1, 1, 1], [0, 1, 1, 1]]
            .into_iter()
            .map(|l| l.to_vec())
            .collect::<Vec<Vec<i32>>>();
        assert_eq!(15, Solution::count_squares(matrix));
    }

    #[test]
    fn test_count_squares_2() {
        let matrix = [[1, 0, 1], [1, 1, 0], [1, 1, 0]]
            .into_iter()
            .map(|l| l.to_vec())
            .collect::<Vec<Vec<i32>>>();
        assert_eq!(7, Solution::count_squares(matrix));
    }
}
