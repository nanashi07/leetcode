// 1594. Maximum Non Negative Product in a Matrix
// https://leetcode.com/problems/maximum-non-negative-product-in-a-matrix/

struct Solution;

impl Solution {
    pub fn max_product_path(grid: Vec<Vec<i32>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::maximum_non_negative_product_in_a_matrix::Solution;

    #[test]
    fn test_max_product_path_1() {
        let grid = [[-1, -2, -3], [-2, -3, -3], [-3, -3, -2]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(-1, Solution::max_product_path(grid));
    }

    #[test]
    fn test_max_product_path_2() {
        let grid = [[1, -2, 1], [1, -2, 1], [3, -4, 1]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(8, Solution::max_product_path(grid));
    }

    #[test]
    fn test_max_product_path_3() {
        let grid = [[1, 3], [0, -4]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(0, Solution::max_product_path(grid));
    }
}
