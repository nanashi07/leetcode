// 3841. Palindromic Path Queries in a Tree
// https://leetcode.com/problems/palindromic-path-queries-in-a-tree/

struct Solution;

/// Segment tree over XOR masks: range XOR update on the closed interval `[l, r]`
/// and point query. Only lazy tags are stored, a leaf value is the XOR of its path.
struct XorSegTree {
    size: usize,
    tree: Vec<i32>,
}

impl XorSegTree {
    fn new(n: usize) -> Self {
        let size = n.next_power_of_two().max(1);
        Self {
            size,
            tree: vec![0; 2 * size],
        }
    }

    fn range_xor(&mut self, l: i32, r: i32, val: i32) {
        let mut left = l as usize + self.size;
        let mut right = r as usize + 1 + self.size;
        while left < right {
            if left & 1 == 1 {
                self.tree[left] ^= val;
                left += 1;
            }
            if right & 1 == 1 {
                right -= 1;
                self.tree[right] ^= val;
            }
            left >>= 1;
            right >>= 1;
        }
    }

    fn get(&self, idx: i32) -> i32 {
        let mut node = idx as usize + self.size;
        let mut res = 0;
        while node > 0 {
            res ^= self.tree[node];
            node >>= 1;
        }
        res
    }
}

impl Solution {
    pub fn palindrome_path(
        n: i32,
        edges: Vec<Vec<i32>>,
        s: String,
        queries: Vec<String>,
    ) -> Vec<bool> {
        let n = n as usize;
        let mut chars = s.into_bytes();

        // Adjacency lists of the tree.
        let mut deg = vec![0usize; n];
        for e in &edges {
            deg[e[0] as usize] += 1;
            deg[e[1] as usize] += 1;
        }
        let mut adj: Vec<Vec<usize>> = deg.into_iter().map(Vec::with_capacity).collect();
        for e in &edges {
            let (a, b) = (e[0] as usize, e[1] as usize);
            adj[a].push(b);
            adj[b].push(a);
        }

        // Binary lifting table: `up[v * log + j]` is the 2^j-th ancestor of `v`.
        // Enough jump bits for any depth `< n`, plus one spare level.
        let mut log = 1usize;
        while 1usize << log <= n {
            log += 1;
        }
        log += 1;
        let mut up = vec![usize::MAX; n * log];
        let mut depth = vec![0usize; n];
        let mut tin = vec![0i32; n];
        let mut tout = vec![0i32; n];
        // Parity mask of the root-to-`v` path (inclusive) using the initial letters.
        let mut base_mask = vec![0i32; n];

        let bit = |c: u8| 1i32 << (c - b'a');
        base_mask[0] = bit(chars[0]);

        // Iterative DFS (Euler tour) so chain-shaped trees cannot blow the stack.
        {
            let mut stack: Vec<(usize, bool)> = Vec::with_capacity(2 * n);
            stack.push((0, false));
            let mut timer = 0i32;
            while let Some((v, left)) = stack.pop() {
                if left {
                    tout[v] = timer - 1;
                    continue;
                }
                tin[v] = timer;
                timer += 1;
                stack.push((v, true));
                for &to in adj[v].iter().rev() {
                    if up[v * log] == to {
                        continue;
                    }
                    up[to * log] = v;
                    depth[to] = depth[v] + 1;
                    base_mask[to] = base_mask[v] ^ bit(chars[to]);
                    stack.push((to, false));
                }
            }
        }

        for j in 1..log {
            for v in 0..n {
                let mid = up[v * log + j - 1];
                if mid != usize::MAX {
                    up[v * log + j] = up[mid * log + j - 1];
                }
            }
        }

        // `delta.get(tin[x])` is the XOR correction of every update applied to an
        // ancestor of `x` (its own subtree contains `x`).
        let mut delta = XorSegTree::new(n);
        let mut ans = Vec::with_capacity(queries.len());

        for query in &queries {
            let args: Vec<&str> = query.split_whitespace().collect();
            if args[0].as_bytes()[0] == b'u' {
                let v: usize = args[1].parse().unwrap_or_default();
                let new_bit = bit(args[2].as_bytes()[0]);
                let old_bit = bit(chars[v]);
                if new_bit != old_bit {
                    chars[v] = args[2].as_bytes()[0];
                    delta.range_xor(tin[v], tout[v], new_bit ^ old_bit);
                }
            } else {
                let u: usize = args[1].parse().unwrap_or_default();
                let v: usize = args[2].parse().unwrap_or_default();

                // Lowest common ancestor of `u` and `v`.
                let (mut a, mut b) = if depth[u] >= depth[v] { (u, v) } else { (v, u) };
                let diff = depth[a] - depth[b];
                for j in 0..log {
                    if (diff >> j) & 1 == 1 {
                        a = up[a * log + j];
                    }
                }
                if a != b {
                    for j in (0..log).rev() {
                        let (pa, pb) = (up[a * log + j], up[b * log + j]);
                        if pa != pb {
                            a = pa;
                            b = pb;
                        }
                    }
                    a = up[a * log];
                }
                let lca = a;

                let mask_u = base_mask[u] ^ delta.get(tin[u]);
                let mask_v = base_mask[v] ^ delta.get(tin[v]);
                // `lca` is excluded twice by the two root paths, so add it back once.
                let mask = mask_u ^ mask_v ^ bit(chars[lca]);
                ans.push(mask & (mask - 1) == 0);
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::palindromic_path_queries_in_a_tree::Solution;
    use crate::shared::vec2d::{to_string_vec, to_vec2d};

    #[test]
    fn test_palindrome_path_1() {
        let n = 3;
        let edges = to_vec2d([[0, 1], [1, 2]]);
        let s = "aac".to_string();
        let queries = to_string_vec(["query 0 2", "update 1 b", "query 0 2"]);
        assert_eq!(
            vec![true, false],
            Solution::palindrome_path(n, edges, s, queries)
        );
    }

    #[test]
    fn test_palindrome_path_2() {
        let n = 4;
        let edges = to_vec2d([[0, 1], [0, 2], [0, 3]]);
        let s = "abca".to_string();
        let queries = to_string_vec([
            "query 1 2",
            "update 0 b",
            "query 2 3",
            "update 3 a",
            "query 1 3",
        ]);
        assert_eq!(
            vec![false, false, true],
            Solution::palindrome_path(n, edges, s, queries)
        );
    }
}
