// 2872. Maximum Number of K-Divisible Components
// https://leetcode.com/problems/maximum-number-of-k-divisible-components/

struct Solution;

impl Solution {
    pub fn max_k_divisible_components(
        n: i32,
        edges: Vec<Vec<i32>>,
        values: Vec<i32>,
        k: i32,
    ) -> i32 {
        let n = n as usize;

        // Build adjacency list
        let mut graph = vec![vec![]; n];
        for edge in edges.iter() {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            graph[u].push(v);
            graph[v].push(u);
        }

        let mut components = 0;

        // DFS to calculate subtree sums and count valid components
        fn dfs(
            node: usize,
            parent: Option<usize>,
            graph: &Vec<Vec<usize>>,
            values: &Vec<i32>,
            k: i32,
            components: &mut i32,
        ) -> i64 {
            let mut subtree_sum = values[node] as i64;

            // Visit all children
            for &child in &graph[node] {
                if Some(child) != parent {
                    let child_sum = dfs(child, Some(node), graph, values, k, components);
                    subtree_sum += child_sum;
                }
            }

            // If this subtree sum is divisible by k, we can split it off
            // (except for the root which we'll count separately)
            if subtree_sum % (k as i64) == 0 {
                *components += 1;
                return 0; // Reset sum as this subtree is now a separate component
            }

            subtree_sum
        }

        dfs(0, None, &graph, &values, k, &mut components);

        components
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::maximum_number_of_k_divisible_components::Solution;

    #[test]
    fn test_max_k_divisible_components_1() {
        let n = 5;
        let edges = [[0, 2], [1, 2], [1, 3], [2, 4]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        let values = [1, 8, 1, 4, 4].to_vec();
        let k = 6;
        assert_eq!(2, Solution::max_k_divisible_components(n, edges, values, k));
    }

    #[test]
    fn test_max_k_divisible_components_2() {
        let n = 7;
        let edges = [[0, 1], [0, 2], [1, 3], [1, 4], [2, 5], [2, 6]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        let values = [3, 0, 6, 1, 5, 2, 1].to_vec();
        let k = 3;
        assert_eq!(3, Solution::max_k_divisible_components(n, edges, values, k));
    }
}
