// # 1277. Count Square Submatrices with All Ones
// https://leetcode.com/problems/count-square-submatrices-with-all-ones/

struct Solution;

impl Solution {
    pub fn count_squares(matrix: Vec<Vec<i32>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::count_square_submatrices_with_all_ones::Solution;

    #[test]
    fn test_count_squares_1() {
        let matrix = [[0, 1, 1, 1], [1, 1, 1, 1], [0, 1, 1, 1]]
            .into_iter()
            .map(|l| l.to_vec())
            .collect::<Vec<Vec<i32>>>();
        assert_eq!(15, Solution::count_squares(matrix));
    }

    #[test]
    fn test_count_squares_2() {
        let matrix = [[1, 0, 1], [1, 1, 0], [1, 1, 0]]
            .into_iter()
            .map(|l| l.to_vec())
            .collect::<Vec<Vec<i32>>>();
        assert_eq!(7, Solution::count_squares(matrix));
    }
}
