// 1886. Determine Whether Matrix Can Be Obtained By Rotation
// https://leetcode.com/problems/determine-whether-matrix-can-be-obtained-by-rotation/

struct Solution;

impl Solution {
    pub fn find_rotation(mat: Vec<Vec<i32>>, target: Vec<Vec<i32>>) -> bool {
        let n = mat.len();
        // Track which rotations (0°, 90°, 180°, 270° clockwise) still match
        let mut ok = [true; 4];
        for i in 0..n {
            for j in 0..n {
                let v = mat[i][j];
                if ok[0] && target[i][j] != v { ok[0] = false; }
                if ok[1] && target[j][n - 1 - i] != v { ok[1] = false; }
                if ok[2] && target[n - 1 - i][n - 1 - j] != v { ok[2] = false; }
                if ok[3] && target[n - 1 - j][i] != v { ok[3] = false; }
            }
        }
        ok.iter().any(|&x| x)
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::determine_whether_matrix_can_be_obtained_by_rotation::Solution;
    use crate::shared::vec2d::to_vec2d;

    #[test]
    fn test_find_rotation_1() {
        let mat = to_vec2d([[0, 1], [1, 0]]);
        let target = to_vec2d([[1, 0], [0, 1]]);
        assert_eq!(true, Solution::find_rotation(mat, target));
    }

    #[test]
    fn test_find_rotation_2() {
        let mat = to_vec2d([[0, 1], [1, 1]]);
        let target = to_vec2d([[1, 0], [0, 1]]);
        assert_eq!(false, Solution::find_rotation(mat, target));
    }

    #[test]
    fn test_find_rotation_3() {
        let mat = to_vec2d([[0, 0, 0], [0, 1, 0], [1, 1, 1]]);
        let target = to_vec2d([[1, 1, 1], [0, 1, 0], [0, 0, 0]]);
        assert_eq!(true, Solution::find_rotation(mat, target));
    }
}
