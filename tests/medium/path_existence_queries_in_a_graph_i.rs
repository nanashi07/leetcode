// 3532. Path Existence Queries in a Graph I
// https://leetcode.com/problems/path-existence-queries-in-a-graph-i/

struct Solution;

impl Solution {
    pub fn path_existence_queries(
        n: i32,
        nums: Vec<i32>,
        max_diff: i32,
        queries: Vec<Vec<i32>>,
    ) -> Vec<bool> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::path_existence_queries_in_a_graph_i::Solution;
    use crate::shared::vec2d::to_vec2d;

    #[test]
    fn test_path_existence_queries_1() {
        let n = 2;
        let nums = [1, 3].to_vec();
        let max_diff = 1;
        let queries = to_vec2d([[0, 0], [0, 1]]);
        assert_eq!(
            [true, false].to_vec(),
            Solution::path_existence_queries(n, nums, max_diff, queries)
        );
    }

    #[test]
    fn test_path_existence_queries_2() {
        let n = 4;
        let nums = [2, 5, 6, 8].to_vec();
        let max_diff = 2;
        let queries = to_vec2d([[0, 1], [0, 2], [1, 3], [2, 3]]);
        assert_eq!(
            [false, false, true, true].to_vec(),
            Solution::path_existence_queries(n, nums, max_diff, queries)
        );
    }
}
