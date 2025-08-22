// # 1504. Count Submatrices With All Ones
// https://leetcode.com/problems/count-submatrices-with-all-ones/

struct Solution;

impl Solution {
    pub fn num_submat(mat: Vec<Vec<i32>>) -> i32 {
        if mat.is_empty() || mat[0].is_empty() {
            return 0;
        }

        let m = mat.len();
        let n = mat[0].len();
        let mut heights = vec![0; n];
        let mut result = 0;

        for i in 0..m {
            // Update heights for current row
            for j in 0..n {
                if mat[i][j] == 1 {
                    heights[j] += 1;
                } else {
                    heights[j] = 0;
                }
            }

            // Count submatrices ending at row i
            result += Self::count_rectangles_in_histogram(&heights);
        }

        result
    }

    fn count_rectangles_in_histogram(heights: &[i32]) -> i32 {
        let n = heights.len();
        let mut count = 0;

        // For each position i, count rectangles ending at position i
        for i in 0..n {
            let mut min_height = heights[i];

            // Extend rectangle leftward from position i
            for j in (0..=i).rev() {
                min_height = min_height.min(heights[j]);
                // Add min_height rectangles of different heights (1 to min_height)
                // ending at position i and starting at position j
                count += min_height;
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::count_submatrices_with_all_ones::Solution;

    #[test]
    fn test_num_submat_1() {
        let mat = [[1, 0, 1], [1, 1, 0], [1, 1, 0]]
            .into_iter()
            .map(|l| l.to_vec())
            .collect::<Vec<Vec<i32>>>();
        assert_eq!(13, Solution::num_submat(mat));
    }

    #[test]
    fn test_num_submat_2() {
        let mat = [[0, 1, 1, 0], [0, 1, 1, 1], [1, 1, 1, 0]]
            .into_iter()
            .map(|l| l.to_vec())
            .collect::<Vec<Vec<i32>>>();
        assert_eq!(24, Solution::num_submat(mat));
    }
}
