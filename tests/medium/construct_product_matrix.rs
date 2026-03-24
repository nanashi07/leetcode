// 2906. Construct Product Matrix
// https://leetcode.com/problems/construct-product-matrix/

struct Solution;

impl Solution {
    pub fn construct_product_matrix(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::construct_product_matrix::Solution;

    #[test]
    fn test_construct_product_matrix_1() {
        let grid = [[1, 2], [3, 4]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        let output = [[24, 12], [8, 6]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(output, Solution::construct_product_matrix(grid));
    }

    #[test]
    fn test_construct_product_matrix_2() {
        let grid = [[12345], [2], [1]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        let output = [[2], [0], [0]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(output, Solution::construct_product_matrix(grid));
    }
}
