// 1292. Maximum Side Length of a Square with Sum Less than or Equal to Threshold
// https://leetcode.com/problems/maximum-side-length-of-a-square-with-sum-less-than-or-equal-to-threshold/

struct Solution;

impl Solution {
    pub fn max_side_length(mat: Vec<Vec<i32>>, threshold: i32) -> i32 {
        let m = mat.len();
        let n = mat[0].len();

        // Build prefix sum matrix
        // prefix[i][j] = sum of elements from (0,0) to (i-1,j-1)
        let mut prefix = vec![vec![0; n + 1]; m + 1];

        for i in 1..=m {
            for j in 1..=n {
                prefix[i][j] =
                    mat[i - 1][j - 1] + prefix[i - 1][j] + prefix[i][j - 1] - prefix[i - 1][j - 1];
            }
        }

        // Helper function to get sum of square with top-left at (r, c) and side length k
        let get_square_sum = |r: usize, c: usize, k: usize| -> i32 {
            prefix[r + k][c + k] - prefix[r][c + k] - prefix[r + k][c] + prefix[r][c]
        };

        // Try from largest possible square to smallest
        let max_k = m.min(n);

        for k in (1..=max_k).rev() {
            // Try all possible positions for a square of size k
            for i in 0..=m - k {
                for j in 0..=n - k {
                    if get_square_sum(i, j, k) <= threshold {
                        return k as i32;
                    }
                }
            }
        }

        0
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::maximum_side_length_of_a_square_with_sum_less_than_or_equal_to_threshold::Solution;

    #[test]
    fn test_max_side_length_1() {
        let mat = [
            [1, 1, 3, 2, 4, 3, 2],
            [1, 1, 3, 2, 4, 3, 2],
            [1, 1, 3, 2, 4, 3, 2],
        ]
        .iter()
        .map(|l| l.to_vec())
        .collect::<Vec<_>>();
        let threshold = 4;
        assert_eq!(2, Solution::max_side_length(mat, threshold));
    }

    #[test]
    fn test_max_side_length_2() {
        let mat = [
            [2, 2, 2, 2, 2],
            [2, 2, 2, 2, 2],
            [2, 2, 2, 2, 2],
            [2, 2, 2, 2, 2],
            [2, 2, 2, 2, 2],
        ]
        .iter()
        .map(|l| l.to_vec())
        .collect::<Vec<_>>();
        let threshold = 1;
        assert_eq!(0, Solution::max_side_length(mat, threshold));
    }
}
