// # 3495. Minimum Operations to Make Array Elements Zero
// https://leetcode.com/problems/minimum-operations-to-make-array-elements-zero/

struct Solution;

impl Solution {
    pub fn min_operations(queries: Vec<Vec<i32>>) -> i64 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::minimum_operations_to_make_array_elements_zero::Solution;

    #[test]
    fn test_min_operations_1() {
        let queries = [[1, 2], [2, 4]]
            .into_iter()
            .map(|l| l.to_vec())
            .collect::<Vec<Vec<i32>>>();
        assert_eq!(3, Solution::min_operations(queries));
    }

    #[test]
    fn test_min_operations_2() {
        let queries = [[2, 6]]
            .into_iter()
            .map(|l| l.to_vec())
            .collect::<Vec<Vec<i32>>>();
        assert_eq!(4, Solution::min_operations(queries));
    }
}
