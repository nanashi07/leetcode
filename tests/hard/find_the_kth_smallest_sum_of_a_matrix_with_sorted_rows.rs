// # 1439. Find the Kth Smallest Sum of a Matrix With Sorted Rows
// https://leetcode.com/problems/find-the-kth-smallest-sum-of-a-matrix-with-sorted-rows/

struct Solution;

impl Solution {
    pub fn kth_smallest(mat: Vec<Vec<i32>>, k: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::find_the_kth_smallest_sum_of_a_matrix_with_sorted_rows::Solution;

    #[test]
    fn test_kth_smallest_1() {
        let mat = vec![vec![1, 3, 11], vec![2, 4, 6]];
        let k = 5;
        assert_eq!(7, Solution::kth_smallest(mat, k));
    }

    #[test]
    fn test_kth_smallest_2() {
        let mat = vec![vec![1, 3, 11], vec![2, 4, 6]];
        let k = 9;
        assert_eq!(17, Solution::kth_smallest(mat, k));
    }

    #[test]
    fn test_kth_smallest_3() {
        let mat = vec![vec![1, 10, 10], vec![1, 4, 5], vec![2, 3, 6]];
        let k = 7;
        assert_eq!(9, Solution::kth_smallest(mat, k));
    }
}
