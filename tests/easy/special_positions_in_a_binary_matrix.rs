// 1582. Special Positions in a Binary Matrix
// https://leetcode.com/problems/special-positions-in-a-binary-matrix/

struct Solution;

impl Solution {
    pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
        let mut n = 0;
        for i in 0..mat.len() {
            let mut col = 0;
            let mut c = 0;
            for j in 0..mat[i].len() {
                if mat[i][j] > 0 {
                    col += mat[i][j];
                    c = j;
                }
            }

            if col == 1 {
                let mut row = 0;
                for i2 in 0..mat.len() {
                    row += mat[i2][c];
                }

                if row == 1 {
                    n += 1;
                }
            }
        }

        n
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::special_positions_in_a_binary_matrix::Solution;

    #[test]
    fn test_num_special_1() {
        let mat = [[1, 0, 0], [0, 0, 1], [1, 0, 0]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(1, Solution::num_special(mat));
    }

    #[test]
    fn test_num_special_2() {
        let mat = [[1, 0, 0], [0, 1, 0], [0, 0, 1]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(3, Solution::num_special(mat));
    }
}
