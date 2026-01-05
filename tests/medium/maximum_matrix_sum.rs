// 1975. Maximum Matrix Sum
// https://leetcode.com/problems/maximum-matrix-sum/

struct Solution;

impl Solution {
    pub fn max_matrix_sum(matrix: Vec<Vec<i32>>) -> i64 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::maximum_matrix_sum::Solution;

    #[test]
    fn test_max_matrix_sum_1() {
        let matrix = [[1, -1], [-1, 1]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(4, Solution::max_matrix_sum(matrix));
    }

    #[test]
    fn test_max_matrix_sum_2() {
        let matrix = [[1, 2, 3], [-1, -2, -3], [1, 2, 3]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(16, Solution::max_matrix_sum(matrix));
    }
}
