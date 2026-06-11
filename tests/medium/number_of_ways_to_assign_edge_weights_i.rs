// 3558. Number of Ways to Assign Edge Weights I
// https://leetcode.com/problems/number-of-ways-to-assign-edge-weights-i/

struct Solution;

impl Solution {
    pub fn assign_edge_weights(edges: Vec<Vec<i32>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::number_of_ways_to_assign_edge_weights_i::Solution;
    use crate::shared::vec2d::to_vec2d;

    #[test]
    fn test_assign_edge_weights_1() {
        let edges = to_vec2d([[1, 2]]);
        assert_eq!(1, Solution::assign_edge_weights(edges));
    }

    #[test]
    fn test_assign_edge_weights_2() {
        let edges = to_vec2d([[1, 2], [1, 3], [3, 4], [3, 5]]);
        assert_eq!(2, Solution::assign_edge_weights(edges));
    }
}
