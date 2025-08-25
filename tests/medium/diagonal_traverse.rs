// # 498. Diagonal Traverse
// https://leetcode.com/problems/diagonal-traverse/

struct Solution;

impl Solution {
    pub fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::diagonal_traverse::Solution;

    #[test]
    fn test_find_diagonal_order_1() {
        let mat = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
            .into_iter()
            .map(|l| l.to_vec())
            .collect::<Vec<Vec<i32>>>();
        assert_eq!(
            [1, 2, 4, 7, 5, 3, 6, 8, 9].to_vec(),
            Solution::find_diagonal_order(mat)
        );
    }

    #[test]
    fn test_find_diagonal_order_2() {
        let mat = [[1, 2], [3, 4]]
            .into_iter()
            .map(|l| l.to_vec())
            .collect::<Vec<Vec<i32>>>();
        assert_eq!([1, 2, 3, 4].to_vec(), Solution::find_diagonal_order(mat));
    }
}
