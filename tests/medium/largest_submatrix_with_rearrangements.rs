// 1727. Largest Submatrix With Rearrangements
// https://leetcode.com/problems/largest-submatrix-with-rearrangements/

struct Solution;

impl Solution {
    pub fn largest_submatrix(matrix: Vec<Vec<i32>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::largest_submatrix_with_rearrangements::Solution;

    #[test]
    fn test_largest_submatrix_1() {
        let matrix = [[0, 0, 1], [1, 1, 1], [1, 0, 1]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(4, Solution::largest_submatrix(matrix));
    }

    #[test]
    fn test_largest_submatrix_2() {
        let matrix = [[1, 0, 1, 0, 1]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(3, Solution::largest_submatrix(matrix));
    }

    #[test]
    fn test_largest_submatrix_3() {
        let matrix = [[1, 1, 0], [1, 0, 1]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(2, Solution::largest_submatrix(matrix));
    }
}
