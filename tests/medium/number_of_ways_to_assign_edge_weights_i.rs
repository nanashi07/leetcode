// 3558. Number of Ways to Assign Edge Weights I
// https://leetcode.com/problems/number-of-ways-to-assign-edge-weights-i/

struct Solution;

impl Solution {
    pub fn assign_edge_weights(edges: Vec<Vec<i32>>) -> i32 {
        const MOD: i64 = 1_000_000_007;

        let n = edges.len() + 1;
        let mut graph = vec![Vec::new(); n + 1];

        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            graph[u].push(v);
            graph[v].push(u);
        }

        let mut seen = vec![false; n + 1];
        let mut queue = std::collections::VecDeque::from([1usize]);
        seen[1] = true;

        let mut levels = 0;
        while !queue.is_empty() {
            let level_size = queue.len();
            for _ in 0..level_size {
                let u = queue.pop_front().unwrap();
                for &v in &graph[u] {
                    if !seen[v] {
                        seen[v] = true;
                        queue.push_back(v);
                    }
                }
            }
            levels += 1;
        }

        if levels <= 1 {
            return 1;
        }

        let mut base = 2_i64;
        let mut exp = (levels - 2) as i32;
        let mut result = 1_i64;
        while exp > 0 {
            if exp & 1 == 1 {
                result = result * base % MOD;
            }
            base = base * base % MOD;
            exp >>= 1;
        }

        result as i32
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
