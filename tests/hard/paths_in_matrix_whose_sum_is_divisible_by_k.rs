// 2435. Paths in Matrix Whose Sum Is Divisible by K
// https://leetcode.com/problems/paths-in-matrix-whose-sum-is-divisible-by-k/

struct Solution;

impl Solution {
    pub fn number_of_paths(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::paths_in_matrix_whose_sum_is_divisible_by_k::Solution;

    #[test]
    fn test_number_of_paths_1() {
        let grid = [[5, 2, 4], [3, 0, 5], [0, 7, 2]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        let k = 3;
        assert_eq!(2, Solution::number_of_paths(grid, k));
    }

    #[test]
    fn test_number_of_paths_2() {
        let grid = [[0, 0]].iter().map(|l| l.to_vec()).collect::<Vec<_>>();
        let k = 5;
        assert_eq!(1, Solution::number_of_paths(grid, k));
    }

    #[test]
    fn test_number_of_paths_3() {
        let grid = [[7, 3, 4, 9], [2, 3, 6, 2], [2, 3, 7, 0]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        let k = 1;
        assert_eq!(10, Solution::number_of_paths(grid, k));
    }
}
