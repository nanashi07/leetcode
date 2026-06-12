// 3559. Number of Ways to Assign Edge Weights II
// https://leetcode.com/problems/number-of-ways-to-assign-edge-weights-ii/

struct Solution;

impl Solution {
    pub fn assign_edge_weights(edges: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::number_of_ways_to_assign_edge_weights_ii::Solution;
    use crate::shared::vec2d::to_vec2d;

    #[test]
    fn test_assign_edge_weights_1() {
        let edges = to_vec2d([[1, 2]]);
        let queries = to_vec2d([[1, 1], [1, 2]]);
        assert_eq!(
            [0, 1].to_vec(),
            Solution::assign_edge_weights(edges, queries)
        );
    }

    #[test]
    fn test_assign_edge_weights_2() {
        let edges = to_vec2d([[1, 2], [1, 3], [3, 4], [3, 5]]);
        let queries = to_vec2d([[1, 4], [3, 4], [2, 5]]);
        assert_eq!(
            [2,1,4].to_vec(),
            Solution::assign_edge_weights(edges, queries)
        );
    }
}
