// 48. Rotate Image
// https://leetcode.com/problems/rotate-image/

struct Solution;

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        // Transpose
        for i in 0..n {
            for j in i + 1..n {
                let tmp = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = tmp;
            }
        }
        // Reverse each row
        for row in matrix.iter_mut() {
            row.reverse();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::rotate_image::Solution;
    use crate::shared::vec2d::to_vec2d;

    #[test]
    fn test_rotate_1() {
        let mut matrix = to_vec2d([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
        let output = to_vec2d([[7, 4, 1], [8, 5, 2], [9, 6, 3]]);
        Solution::rotate(&mut matrix);
        assert_eq!(output, matrix);
    }

    #[test]
    fn test_rotate_2() {
        let mut matrix = to_vec2d([
            [5, 1, 9, 11],
            [2, 4, 8, 10],
            [13, 3, 6, 7],
            [15, 14, 12, 16],
        ]);
        let output = to_vec2d([
            [15, 13, 2, 5],
            [14, 3, 4, 1],
            [12, 6, 8, 9],
            [16, 7, 10, 11],
        ]);
        Solution::rotate(&mut matrix);
        assert_eq!(output, matrix);
    }
}
