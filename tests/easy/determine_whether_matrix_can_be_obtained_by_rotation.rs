// 1886. Determine Whether Matrix Can Be Obtained By Rotation
// https://leetcode.com/problems/determine-whether-matrix-can-be-obtained-by-rotation/

struct Solution;

impl Solution {
    pub fn find_rotation(mat: Vec<Vec<i32>>, target: Vec<Vec<i32>>) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::determine_whether_matrix_can_be_obtained_by_rotation::Solution;

    #[test]
    fn test_find_rotation_1() {
        let mat = [[0, 1], [1, 0]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        let target = [[1, 0], [0, 1]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(true, Solution::find_rotation(mat, target));
    }

    #[test]
    fn test_find_rotation_2() {
        let mat = [[0, 1], [1, 1]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        let target = [[1, 0], [0, 1]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(false, Solution::find_rotation(mat, target));
    }

    #[test]
    fn test_find_rotation_3() {
        let mat = [[0, 0, 0], [0, 1, 0], [1, 1, 1]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        let target = [[1, 1, 1], [0, 1, 0], [0, 0, 0]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(true, Solution::find_rotation(mat, target));
    }
}
