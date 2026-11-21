// 3331. Find Subtree Sizes After Changes
// https://leetcode.com/problems/find-subtree-sizes-after-changes/

struct Solution;

impl Solution {
    pub fn find_subtree_sizes(parent: Vec<i32>, s: String) -> Vec<i32> {
        let n = parent.len();
        let bytes = s.as_bytes();

        // Original tree as a flat adjacency list (CSR): children[start[u]..start[u + 1]).
        let mut degree = vec![0usize; n];
        for &p in parent.iter().skip(1) {
            degree[p as usize] += 1;
        }
        let mut start = vec![0usize; n + 1];
        for (i, &d) in degree.iter().enumerate() {
            start[i + 1] = start[i] + d;
        }
        let mut children = vec![0usize; n - 1];
        let mut cursor = start.clone();
        for (i, &p) in parent.iter().enumerate().skip(1) {
            let p = p as usize;
            children[cursor[p]] = i;
            cursor[p] += 1;
        }

        // Iterative DFS carrying `last[c]`: the deepest node on the current root path
        // whose character is c. That is exactly the closest ancestor y with s[y] == s[x],
        // so it gives every node's final parent in one pass.
        let mut last = [-1i32; 26];
        let mut final_parent = vec![0usize; n];
        // Frame: (node, next child offset, saved value of last[char(node)]).
        let mut stack: Vec<(usize, usize, i32)> = Vec::with_capacity(n);
        let root_char = (bytes[0] - b'a') as usize;
        let saved_root = last[root_char];
        last[root_char] = 0;
        stack.push((0, start[0], saved_root));

        while let Some((u, next, saved)) = stack.pop() {
            if next < start[u + 1] {
                let v = children[next];
                stack.push((u, next + 1, saved));

                let c = (bytes[v] - b'a') as usize;
                let ancestor = last[c];
                final_parent[v] = if ancestor >= 0 {
                    ancestor as usize
                } else {
                    parent[v] as usize
                };
                last[c] = v as i32;
                stack.push((v, start[v], ancestor));
            } else {
                last[(bytes[u] - b'a') as usize] = saved;
            }
        }

        // Subtree sizes of the final tree: accumulate leaves upward (topological order is
        // guaranteed because every node's new parent has strictly smaller depth).
        let mut pending = vec![0usize; n];
        for &p in final_parent.iter().skip(1) {
            pending[p] += 1;
        }
        let mut size = vec![1i32; n];
        let mut queue: Vec<usize> = (0..n).filter(|&u| pending[u] == 0).collect();
        while let Some(u) = queue.pop() {
            if u != 0 {
                let p = final_parent[u];
                size[p] += size[u];
                pending[p] -= 1;
                if pending[p] == 0 {
                    queue.push(p);
                }
            }
        }

        size
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::find_subtree_sizes_after_changes::Solution;

    #[test]
    fn test_find_subtree_sizes_1() {
        let parent: Vec<i32> = vec![-1, 0, 0, 1, 1, 1];
        let s: String = "abaabc".to_string();
        assert_eq!(
            [6, 3, 1, 1, 1, 1].to_vec(),
            Solution::find_subtree_sizes(parent, s)
        );
    }

    #[test]
    fn test_find_subtree_sizes_2() {
        let parent: Vec<i32> = vec![-1, 0, 4, 0, 1];
        let s: String = "abbba".to_string();
        assert_eq!(
            [5, 2, 1, 1, 1].to_vec(),
            Solution::find_subtree_sizes(parent, s)
        );
    }
}
