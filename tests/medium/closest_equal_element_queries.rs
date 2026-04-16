// 3488. Closest Equal Element Queries
// https://leetcode.com/problems/closest-equal-element-queries/

struct Solution;

impl Solution {
    pub fn solve_queries(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::closest_equal_element_queries::Solution;

    #[test]
    fn test_solve_queries_1() {
        let nums = [1, 3, 1, 4, 1, 3, 2].to_vec();
        let queries = [0, 3, 5].to_vec();
        let output = [2, -1, 3].to_vec();
        assert_eq!(output, Solution::solve_queries(nums, queries));
    }

    #[test]
    fn test_solve_queries_2() {
        let nums = [1, 2, 3, 4].to_vec();
        let queries = [0, 1, 2, 3].to_vec();
        let output = [-1, -1, -1, -1].to_vec();
        assert_eq!(output, Solution::solve_queries(nums, queries));
    }
}
