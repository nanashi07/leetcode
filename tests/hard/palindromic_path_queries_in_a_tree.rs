// 3841. Palindromic Path Queries in a Tree
// https://leetcode.com/problems/palindromic-path-queries-in-a-tree/

struct Solution;

impl Solution {
    pub fn palindrome_path(
        n: i32,
        edges: Vec<Vec<i32>>,
        s: String,
        queries: Vec<String>,
    ) -> Vec<bool> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::palindromic_path_queries_in_a_tree::Solution;
    use crate::shared::vec2d::{to_string_vec, to_vec2d};

    #[test]
    fn test_palindrome_path_1() {
        let n = 3;
        let edges = to_vec2d([[0, 1], [1, 2]]);
        let s = "aac".to_string();
        let queries = to_string_vec(["query 0 2", "update 1 b", "query 0 2"]);
        assert_eq!(
            vec![true, false],
            Solution::palindrome_path(n, edges, s, queries)
        );
    }

    #[test]
    fn test_palindrome_path_2() {
        let n = 4;
        let edges = to_vec2d([[0, 1], [0, 2], [0, 3]]);
        let s = "abca".to_string();
        let queries = to_string_vec([
            "query 1 2",
            "update 0 b",
            "query 2 3",
            "update 3 a",
            "query 1 3",
        ]);
        assert_eq!(
            vec![false, false, true],
            Solution::palindrome_path(n, edges, s, queries)
        );
    }
}
