// 3161. Block Placement Queries
// https://leetcode.com/problems/block-placement-queries/
//
// Model: number line with position 0 as a permanent wall. Obstacles are points.
// A block of size sz fits between two consecutive walls/obstacles at positions a
// and b iff b - a >= sz. For query [2, x, sz], x acts as a virtual right boundary,
// so the answer is max(max_gap_within_[0,x], x - last_obstacle_in_[0,x]) >= sz.
//
// Offline O(n log n): pre-block all type-1 positions and process queries in
// reverse so that type-1 queries become unblock operations. A segment tree
// tracks max gaps between blocked positions.

struct Solution;

#[derive(Clone)]
struct Node {
    max_gap: i32,   // max gap between consecutive blocked positions in this segment
    left_dist: i32, // dist from left end to first blocked pos (= len if no blocks)
    right_dist: i32,// dist from last blocked pos to right end (= len if no blocks)
    has_block: bool,
    len: i32,
}

impl Node {
    fn merge(a: &Node, b: &Node) -> Node {
        let len = a.len + b.len;
        let left_dist = if a.has_block { a.left_dist } else { a.len + b.left_dist };
        let right_dist = if b.has_block { b.right_dist } else { b.len + a.right_dist };
        let has_block = a.has_block || b.has_block;
        let max_gap = match (a.has_block, b.has_block) {
            // gap between last block in a and first block in b = a.right_dist + b.left_dist + 1
            (true, true) => a.max_gap.max(b.max_gap).max(a.right_dist + b.left_dist + 1),
            (true, false) => a.max_gap,
            (false, true) => b.max_gap,
            (false, false) => 0,
        };
        Node { max_gap, left_dist, right_dist, has_block, len }
    }
}

struct SegTree {
    tree: Vec<Node>,
}

impl SegTree {
    fn new(n: usize) -> Self {
        let empty = Node { max_gap: 0, left_dist: 0, right_dist: 0, has_block: false, len: 0 };
        SegTree { tree: vec![empty; 4 * n] }
    }

    fn build(&mut self, node: usize, l: usize, r: usize) {
        let len = (r - l + 1) as i32;
        self.tree[node] = Node { max_gap: 0, left_dist: len, right_dist: len, has_block: false, len };
        if l == r { return; }
        let m = (l + r) / 2;
        self.build(2 * node, l, m);
        self.build(2 * node + 1, m + 1, r);
    }

    fn update(&mut self, node: usize, l: usize, r: usize, pos: usize, blocked: bool) {
        if l == r {
            self.tree[node] = Node {
                max_gap: 0,
                left_dist: if blocked { 0 } else { 1 },
                right_dist: if blocked { 0 } else { 1 },
                has_block: blocked,
                len: 1,
            };
            return;
        }
        let m = (l + r) / 2;
        if pos <= m {
            self.update(2 * node, l, m, pos, blocked);
        } else {
            self.update(2 * node + 1, m + 1, r, pos, blocked);
        }
        let merged = Node::merge(&self.tree[2 * node].clone(), &self.tree[2 * node + 1].clone());
        self.tree[node] = merged;
    }

    fn query(&self, node: usize, l: usize, r: usize, ql: usize, qr: usize) -> Node {
        if ql <= l && r <= qr {
            return self.tree[node].clone();
        }
        let m = (l + r) / 2;
        if qr <= m {
            return self.query(2 * node, l, m, ql, qr);
        }
        if ql > m {
            return self.query(2 * node + 1, m + 1, r, ql, qr);
        }
        let ln = self.query(2 * node, l, m, ql, qr);
        let rn = self.query(2 * node + 1, m + 1, r, ql, qr);
        Node::merge(&ln, &rn)
    }
}

impl Solution {
    pub fn get_results(queries: Vec<Vec<i32>>) -> Vec<bool> {
        let max_pos = queries.iter().map(|q| q[1] as usize).max().unwrap_or(0);

        let mut seg = SegTree::new(max_pos + 1);
        seg.build(1, 0, max_pos);

        // Position 0 is a permanent wall (origin boundary).
        seg.update(1, 0, max_pos, 0, true);

        // Pre-block all type-1 obstacle positions (offline: start fully blocked).
        for q in &queries {
            if q[0] == 1 {
                seg.update(1, 0, max_pos, q[1] as usize, true);
            }
        }

        // Process queries in reverse: type-1 becomes unblock, type-2 queries answered.
        let mut results = Vec::new();
        for q in queries.iter().rev() {
            if q[0] == 1 {
                seg.update(1, 0, max_pos, q[1] as usize, false);
            } else {
                let x = q[1] as usize;
                let sz = q[2];
                // right_dist = x - last_obstacle_in_[0,x] = virtual gap to right boundary x.
                let result = seg.query(1, 0, max_pos, 0, x);
                results.push(result.max_gap.max(result.right_dist) >= sz);
            }
        }

        results.reverse();
        results
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::block_placement_queries::Solution;

    #[test]
    fn test_get_results_1() {
        let queries = [
            [1, 2].to_vec(),
            [2, 3, 3].to_vec(),
            [2, 3, 1].to_vec(),
            [2, 2, 2].to_vec(),
        ]
        .to_vec();
        assert_eq!([false, true, true].to_vec(), Solution::get_results(queries));
    }

    #[test]
    fn test_get_results_2() {
        let queries = [
            [1, 7].to_vec(),
            [2, 7, 6].to_vec(),
            [1, 2].to_vec(),
            [2, 7, 5].to_vec(),
            [2, 7, 6].to_vec(),
        ]
        .to_vec();
        assert_eq!([true, true, false].to_vec(), Solution::get_results(queries));
    }
}
