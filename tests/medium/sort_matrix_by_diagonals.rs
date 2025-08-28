// # 3446. Sort Matrix by Diagonals
// https://leetcode.com/problems/sort-matrix-by-diagonals/

struct Solution;

impl Solution {
    pub fn sort_matrix(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::sort_matrix_by_diagonals::Solution;

    #[test]
    fn test_sort_matrix_1() {
        let grid = [[1, 7, 3], [9, 8, 2], [4, 5, 6]]
            .into_iter()
            .map(|l| l.to_vec())
            .collect::<Vec<Vec<i32>>>();
        let output = [[8, 2, 3], [9, 6, 7], [4, 5, 1]]
            .into_iter()
            .map(|l| l.to_vec())
            .collect::<Vec<Vec<i32>>>();
        assert_eq!(output, Solution::sort_matrix(grid));
    }

    #[test]
    fn test_sort_matrix_2() {
        let grid = [[0, 1], [1, 2]]
            .into_iter()
            .map(|l| l.to_vec())
            .collect::<Vec<Vec<i32>>>();
        let output = [[2, 1], [1, 0]]
            .into_iter()
            .map(|l| l.to_vec())
            .collect::<Vec<Vec<i32>>>();
        assert_eq!(output, Solution::sort_matrix(grid));
    }

    #[test]
    fn test_sort_matrix_3() {
        let grid = [[1]]
            .into_iter()
            .map(|l| l.to_vec())
            .collect::<Vec<Vec<i32>>>();
        let output = [[1]]
            .into_iter()
            .map(|l| l.to_vec())
            .collect::<Vec<Vec<i32>>>();
        assert_eq!(output, Solution::sort_matrix(grid));
    }
}
