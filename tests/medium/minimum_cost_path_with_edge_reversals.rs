// 3650. Minimum Cost Path with Edge Reversals
// https://leetcode.com/problems/minimum-cost-path-with-edge-reversals/

struct Solution;

impl Solution {
    pub fn min_cost(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::minimum_cost_path_with_edge_reversals::Solution;

    #[test]
    fn test_min_cost_1() {
        let n = 4;
        let edges = [[0, 1, 3], [3, 1, 1], [2, 3, 4], [0, 2, 2]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(5, Solution::min_cost(n, edges));
    }

    #[test]
    fn test_min_cost_2() {
        let n = 4;
        let edges = [[0, 2, 1], [2, 1, 1], [1, 3, 1], [2, 3, 3]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(3, Solution::min_cost(n, edges));
    }
}
