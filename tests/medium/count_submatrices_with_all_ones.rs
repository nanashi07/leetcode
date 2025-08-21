// # 1504. Count Submatrices With All Ones
// https://leetcode.com/problems/count-submatrices-with-all-ones/

struct Solution;

impl Solution {
    pub fn num_submat(mat: Vec<Vec<i32>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::count_submatrices_with_all_ones::Solution;

    #[test]
    fn test_num_submat_1() {
        let mat = [[1, 0, 1], [1, 1, 0], [1, 1, 0]]
            .into_iter()
            .map(|l| l.to_vec())
            .collect::<Vec<Vec<i32>>>();
        assert_eq!(13, Solution::num_submat(mat));
    }

    #[test]
    fn test_num_submat_2() {
        let mat = [[0, 1, 1, 0], [0, 1, 1, 1], [1, 1, 1, 0]]
            .into_iter()
            .map(|l| l.to_vec())
            .collect::<Vec<Vec<i32>>>();
        assert_eq!(24, Solution::num_submat(mat));
    }
}
