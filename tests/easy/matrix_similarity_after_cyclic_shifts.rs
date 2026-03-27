// 2946. Matrix Similarity After Cyclic Shifts
// https://leetcode.com/problems/matrix-similarity-after-cyclic-shifts/

struct Solution;

impl Solution {
    pub fn are_similar(mat: Vec<Vec<i32>>, k: i32) -> bool {
        for i in 0..mat.len() {
            let l = mat[i].len() as i32;
            let k = k % l;
            for j in 0..mat[i].len() {
                let j2 = if i % 2 == 0 {
                    let j = j as i32 - k;
                    if j < 0 {
                        j + l
                    } else {
                        j
                    }
                } else {
                    let j = j as i32 + k;
                    if j < l {
                        j
                    } else {
                        j - l
                    }
                };
                if mat[i][j] != mat[i][j2 as usize] {
                    return false;
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::matrix_similarity_after_cyclic_shifts::Solution;
    use crate::shared::vec2d::to_vec2d;

    #[test]
    fn test_are_similar_1() {
        let mat = to_vec2d([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
        let k = 4;
        assert_eq!(false, Solution::are_similar(mat, k));
    }

    #[test]
    fn test_are_similar_2() {
        let mat = to_vec2d([[1, 2, 1, 2], [5, 5, 5, 5], [6, 3, 6, 3]]);
        let k = 2;
        assert_eq!(true, Solution::are_similar(mat, k));
    }

    #[test]
    fn test_are_similar_3() {
        let mat = to_vec2d([[2, 2], [2, 2]]);
        let k = 3;
        assert_eq!(true, Solution::are_similar(mat, k));
    }
}
