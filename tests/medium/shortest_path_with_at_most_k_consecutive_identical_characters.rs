// 3970. Shortest Path With At Most K Consecutive Identical Characters
// https://leetcode.com/problems/shortest-path-with-at-most-k-consecutive-identical-characters/

struct Solution;

impl Solution {
    pub fn shortest_path(n: i32, edges: Vec<Vec<i32>>, labels: String, k: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::shortest_path_with_at_most_k_consecutive_identical_characters::Solution;
    use crate::shared::vec2d::to_vec2d;

    #[test]
    fn test_shortest_path_1() {
        let n = 3;
        let edges = to_vec2d([[0, 1, 1], [1, 2, 1], [0, 2, 3]]);
        let labels = "aab".to_string();
        let k = 1;
        assert_eq!(3, Solution::shortest_path(n, edges, labels, k));
    }

    #[test]
    fn test_shortest_path_2() {
        let n = 3;
        let edges = to_vec2d([[0, 1, 1], [1, 2, 1], [0, 2, 3]]);
        let labels = "aab".to_string();
        let k = 2;
        assert_eq!(2, Solution::shortest_path(n, edges, labels, k));
    }

    #[test]
    fn test_shortest_path_3() {
        let n = 3;
        let edges = to_vec2d([[0, 1, 1], [1, 2, 1]]);
        let labels = "aaa".to_string();
        let k = 2;
        assert_eq!(-1, Solution::shortest_path(n, edges, labels, k));
    }
}
