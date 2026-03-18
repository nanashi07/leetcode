// 3070. Count Submatrices with Top-Left Element and Sum Less Than k
// https://leetcode.com/problems/count-submatrices-with-top-left-element-and-sum-less-than-k/

struct Solution;

impl Solution {
    pub fn count_submatrices(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::count_submatrices_with_top_left_element_and_sum_less_than_k::Solution;

    #[test]
    fn test_count_submatrices_1() {
        let grid = [[7, 6, 3], [6, 6, 1]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        let k = 18;
        assert_eq!(4, Solution::count_submatrices(grid, k));
    }

    #[test]
    fn test_count_submatrices_2() {
        let grid = [[7, 2, 9], [1, 5, 0], [2, 6, 6]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        let k = 20;
        assert_eq!(6, Solution::count_submatrices(grid, k));
    }
}
