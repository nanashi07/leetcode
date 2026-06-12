// 3559. Number of Ways to Assign Edge Weights II
// https://leetcode.com/problems/number-of-ways-to-assign-edge-weights-ii/

struct Solution;

impl Solution {
    pub fn assign_edge_weights(edges: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        const MOD: i64 = 1_000_000_007;

        let n = edges.len() + 1;
        let mut graph = vec![Vec::new(); n + 1];

        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            graph[u].push(v);
            graph[v].push(u);
        }

        let mut log = 1usize;
        while (1usize << log) <= n {
            log += 1;
        }

        let mut up = vec![vec![0usize; n + 1]; log];
        let mut depth = vec![0usize; n + 1];
        let mut seen = vec![false; n + 1];

        let mut queue = std::collections::VecDeque::from([1usize]);
        seen[1] = true;
        up[0][1] = 1;

        while let Some(u) = queue.pop_front() {
            for &v in &graph[u] {
                if seen[v] {
                    continue;
                }
                seen[v] = true;
                depth[v] = depth[u] + 1;
                up[0][v] = u;
                queue.push_back(v);
            }
        }

        for k in 1..log {
            for v in 1..=n {
                up[k][v] = up[k - 1][up[k - 1][v]];
            }
        }

        let mut pow2 = vec![1_i64; n + 1];
        for i in 1..=n {
            pow2[i] = (pow2[i - 1] * 2) % MOD;
        }

        let lca = |mut a: usize, mut b: usize, up: &Vec<Vec<usize>>, depth: &Vec<usize>| {
            if depth[a] < depth[b] {
                std::mem::swap(&mut a, &mut b);
            }

            let diff = depth[a] - depth[b];
            for (k, ancestors) in up.iter().enumerate() {
                if ((diff >> k) & 1) == 1 {
                    a = ancestors[a];
                }
            }

            if a == b {
                return a;
            }

            for k in (0..up.len()).rev() {
                if up[k][a] != up[k][b] {
                    a = up[k][a];
                    b = up[k][b];
                }
            }

            up[0][a]
        };

        let mut result = Vec::with_capacity(queries.len());
        for query in queries {
            let u = query[0] as usize;
            let v = query[1] as usize;

            if u == v {
                result.push(0);
                continue;
            }

            let p = lca(u, v, &up, &depth);
            let dist = depth[u] + depth[v] - (depth[p] * 2);
            result.push(pow2[dist - 1] as i32);
        }

        result
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
