// 3970. Shortest Path With At Most K Consecutive Identical Characters
// https://leetcode.com/problems/shortest-path-with-at-most-k-consecutive-identical-characters/

use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    // Dijkstra over the states `(node, length of the trailing run of equal labels)`: which runs are
    // still allowed depends on that length, so a plain per-node distance is not enough (a cheap
    // arrival with a full run can be dead while a pricier one with a short run continues).
    // A state dominates another when it reaches the same node with both a no larger distance and a
    // no longer run, so every node keeps only its Pareto-optimal `(run, distance)` pairs; they are
    // sorted by run with strictly decreasing distances, which keeps each frontier short in practice.
    // Time O((n + m) * p * log((n + m) * p)), space O((n + m) * p), for frontier size p <= k.
    pub fn shortest_path(n: i32, edges: Vec<Vec<i32>>, labels: String, k: i32) -> i32 {
        let n = n as usize;
        let label = labels.as_bytes();

        let mut g = vec![Vec::new(); n];
        for e in &edges {
            let (u, v, w) = (e[0] as usize, e[1] as usize, e[2]);
            g[u].push((v, w));
            g[v].push((u, w));
        }

        // `best[u]` is the Pareto frontier of node `u`; the start node begins with a single label.
        let mut best = vec![Vec::<(i32, i64)>::new(); n];
        best[0].push((1, 0));

        // Min heap of open states: (distance, node, trailing run).
        let mut heap = BinaryHeap::from(vec![Reverse((0i64, 0usize, 1i32))]);

        while let Some(Reverse((dist, u, run))) = heap.pop() {
            // Ignore entries that a later state has dominated and dropped from the frontier.
            if !best[u].iter().any(|&(r, d)| r == run && d == dist) {
                continue;
            }
            // Popped in increasing distance, so this is the cheapest valid path to `n - 1`.
            if u == n - 1 {
                return dist as i32;
            }

            for &(v, w) in &g[u] {
                let next_run = if label[v] == label[u] { run + 1 } else { 1 };
                if next_run > k {
                    continue;
                }
                let next_dist = dist + i64::from(w);
                if Self::improve(&mut best[v], next_run, next_dist) {
                    heap.push(Reverse((next_dist, v, next_run)));
                }
            }
        }

        -1
    }

    // Add `(run, dist)` to a frontier unless an existing pair already reaches that node cheaper or
    // equal with a shorter or equal run; pairs the new one dominates are dropped. `front` stays
    // sorted by run with strictly decreasing distances, so both scans cost O(front.len()).
    fn improve(front: &mut Vec<(i32, i64)>, run: i32, dist: i64) -> bool {
        if front.iter().any(|&(r, d)| r <= run && d <= dist) {
            return false;
        }
        front.retain(|&(r, d)| r < run || d < dist);
        let at = front.partition_point(|&(r, _)| r < run);
        front.insert(at, (run, dist));
        true
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
