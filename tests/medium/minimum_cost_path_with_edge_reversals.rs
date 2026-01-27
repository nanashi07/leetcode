// 3650. Minimum Cost Path with Edge Reversals
// https://leetcode.com/problems/minimum-cost-path-with-edge-reversals/

use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    // https://leetcode.com/problems/minimum-cost-path-with-edge-reversals/editorial/
    pub fn min_cost(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut g = vec![vec![]; n];
        for e in edges {
            let (x, y, w) = (e[0] as usize, e[1] as usize, e[2]);
            g[x].push((y as i32, w));
            g[y].push((x as i32, 2 * w));
        }

        let mut dist = vec![i32::MAX; n];
        let mut visited = vec![false; n];
        let mut heap = BinaryHeap::new(); // max heap, but storing negative values

        dist[0] = 0;
        heap.push((0, 0)); // (-Distance, Node)

        while let Some((neg_d, node)) = heap.pop() {
            let d = -neg_d;
            let node = node as usize;

            if node == n - 1 {
                return d;
            }

            if visited[node] {
                continue;
            }
            visited[node] = true;

            for &(next, weight) in &g[node] {
                let next_idx = next as usize;
                let new_dist = d + weight;
                if new_dist < dist[next_idx] {
                    dist[next_idx] = new_dist;
                    heap.push((-new_dist, next));
                }
            }
        }

        -1
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
