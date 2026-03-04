// 1582. Special Positions in a Binary Matrix
// https://leetcode.com/problems/special-positions-in-a-binary-matrix/

struct Solution;

impl Solution {
    pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
        todo!()
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
