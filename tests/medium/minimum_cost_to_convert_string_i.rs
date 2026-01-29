// 2976. Minimum Cost to Convert String I
// https://leetcode.com/problems/minimum-cost-to-convert-string-i/

struct Solution;

impl Solution {
    pub fn minimum_cost(
        source: String,
        target: String,
        original: Vec<char>,
        changed: Vec<char>,
        cost: Vec<i32>,
    ) -> i64 {
        // Initialize distance matrix with infinity
        // We'll use indices 0-25 for 'a'-'z'
        const INF: i64 = i64::MAX / 2;
        let mut dist = vec![vec![INF; 26]; 26];

        // Distance from a character to itself is 0
        for i in 0..26 {
            dist[i][i] = 0;
        }

        // Build the graph with given edges
        for i in 0..original.len() {
            let from = (original[i] as usize) - ('a' as usize);
            let to = (changed[i] as usize) - ('a' as usize);
            let c = cost[i] as i64;
            // Keep minimum cost if there are multiple edges between same pair
            dist[from][to] = dist[from][to].min(c);
        }

        // Floyd-Warshall algorithm to find shortest paths between all pairs
        for k in 0..26 {
            for i in 0..26 {
                for j in 0..26 {
                    if dist[i][k] != INF && dist[k][j] != INF {
                        dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j]);
                    }
                }
            }
        }

        // Calculate total cost to transform source to target
        let mut total_cost: i64 = 0;
        let source_chars: Vec<char> = source.chars().collect();
        let target_chars: Vec<char> = target.chars().collect();

        for i in 0..source_chars.len() {
            let from = (source_chars[i] as usize) - ('a' as usize);
            let to = (target_chars[i] as usize) - ('a' as usize);

            if dist[from][to] == INF {
                return -1; // Impossible to convert
            }

            total_cost += dist[from][to];
        }

        total_cost
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::minimum_cost_to_convert_string_i::Solution;

    #[test]
    fn test_minimum_cost_1() {
        let source = "abcd".to_string();
        let target = "acbe".to_string();
        let original = ["a", "b", "c", "c", "e", "d"]
            .iter()
            .map(|c| c.chars().next().unwrap())
            .collect::<Vec<_>>();
        let changed = ["b", "c", "b", "e", "b", "e"]
            .iter()
            .map(|c| c.chars().next().unwrap())
            .collect::<Vec<_>>();
        let cost = [2, 5, 5, 1, 2, 20].to_vec();
        assert_eq!(
            28,
            Solution::minimum_cost(source, target, original, changed, cost)
        );
    }

    #[test]
    fn test_minimum_cost_2() {
        let source = "aaaa".to_string();
        let target = "bbbb".to_string();
        let original = ["a", "c"]
            .iter()
            .map(|c| c.chars().next().unwrap())
            .collect::<Vec<_>>();
        let changed = ["c", "b"]
            .iter()
            .map(|c| c.chars().next().unwrap())
            .collect::<Vec<_>>();
        let cost = [1, 2].to_vec();
        assert_eq!(
            12,
            Solution::minimum_cost(source, target, original, changed, cost)
        );
    }

    #[test]
    fn test_minimum_cost_3() {
        let source = "abcd".to_string();
        let target = "abce".to_string();
        let original = ["a"]
            .iter()
            .map(|c| c.chars().next().unwrap())
            .collect::<Vec<_>>();
        let changed = ["e"]
            .iter()
            .map(|c| c.chars().next().unwrap())
            .collect::<Vec<_>>();
        let cost = [10000].to_vec();
        assert_eq!(
            -1,
            Solution::minimum_cost(source, target, original, changed, cost)
        );
    }
}
