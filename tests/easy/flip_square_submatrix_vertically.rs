// 3643. Flip Square Submatrix Vertically
// https://leetcode.com/problems/flip-square-submatrix-vertically/

struct Solution;

impl Solution {
    pub fn reverse_submatrix(grid: Vec<Vec<i32>>, x: i32, y: i32, k: i32) -> Vec<Vec<i32>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::flip_square_submatrix_vertically::Solution;

    #[test]
    fn test_reverse_submatrix_1() {
        let grid = [
            [1, 2, 3, 4],
            [5, 6, 7, 8],
            [9, 10, 11, 12],
            [13, 14, 15, 16],
        ]
        .iter()
        .map(|l| l.to_vec())
        .collect::<Vec<_>>();
        let x = 1;
        let y = 0;
        let k = 3;
        let output = [
            [1, 2, 3, 4],
            [13, 14, 15, 8],
            [9, 10, 11, 12],
            [5, 6, 7, 16],
        ]
        .iter()
        .map(|l| l.to_vec())
        .collect::<Vec<_>>();
        assert_eq!(output, Solution::reverse_submatrix(grid, x, y, k));
    }

    #[test]
    fn test_reverse_submatrix_2() {
        let grid = [[3, 4, 2, 3], [2, 3, 4, 2]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        let x = 0;
        let y = 2;
        let k = 2;
        let output = [[3, 4, 4, 2], [2, 3, 2, 3]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(output, Solution::reverse_submatrix(grid, x, y, k));
    }
}
