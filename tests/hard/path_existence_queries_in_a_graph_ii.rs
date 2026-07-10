// 3534. Path Existence Queries in a Graph II
// https://leetcode.com/problems/path-existence-queries-in-a-graph-ii/

struct Solution;

impl Solution {
    pub fn path_existence_queries(
        n: i32,
        nums: Vec<i32>,
        max_diff: i32,
        queries: Vec<Vec<i32>>,
    ) -> Vec<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::path_existence_queries_in_a_graph_ii::Solution;
    use crate::shared::vec2d::to_vec2d;

    #[test]
    fn test_path_existence_queries_1() {
        let n = 5;
        let nums = [1, 8, 3, 4, 2].to_vec();
        let max_diff = 3;
        let queries = to_vec2d([[0, 3], [2, 4]]);
        assert_eq!(
            [1, 1].to_vec(),
            Solution::path_existence_queries(n, nums, max_diff, queries)
        );
    }

    #[test]
    fn test_path_existence_queries_2() {
        let n = 5;
        let nums = [5, 3, 1, 9, 10].to_vec();
        let max_diff = 2;
        let queries = to_vec2d([[0, 1], [0, 2], [2, 3], [4, 3]]);
        assert_eq!(
            [1, 2, -1, 1].to_vec(),
            Solution::path_existence_queries(n, nums, max_diff, queries)
        );
    }

    #[test]
    fn test_path_existence_queries_3() {
        let n = 3;
        let nums = [3, 6, 1].to_vec();
        let max_diff = 1;
        let queries = to_vec2d([[0, 0], [0, 1], [1, 2]]);
        assert_eq!(
            [0, -1, -1].to_vec(),
            Solution::path_existence_queries(n, nums, max_diff, queries)
        );
    }
}
