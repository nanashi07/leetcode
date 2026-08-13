// 2213. Longest Substring of One Repeating Character
// https://leetcode.com/problems/longest-substring-of-one-repeating-character/

struct Solution;

#[derive(Clone, Copy, Default)]
struct Node {
    len: i32,
    pref_len: i32,
    suff_len: i32,
    max_len: i32,
    pref_char: u8,
    suff_char: u8,
}

impl Node {
    fn new(c: u8) -> Self {
        Self {
            len: 1,
            pref_len: 1,
            suff_len: 1,
            max_len: 1,
            pref_char: c,
            suff_char: c,
        }
    }

    fn merge(left: &Node, right: &Node) -> Self {
        let mut pref_len = left.pref_len;
        if left.pref_len == left.len && left.pref_char == right.pref_char {
            pref_len += right.pref_len;
        }

        let mut suff_len = right.suff_len;
        if right.suff_len == right.len && right.suff_char == left.suff_char {
            suff_len += left.suff_len;
        }

        let mut max_len = left.max_len.max(right.max_len);
        if left.suff_char == right.pref_char {
            max_len = max_len.max(left.suff_len + right.pref_len);
        }

        Self {
            len: left.len + right.len,
            pref_len,
            suff_len,
            max_len,
            pref_char: left.pref_char,
            suff_char: right.suff_char,
        }
    }
}

struct SegTree {
    tree: Vec<Node>,
}

impl SegTree {
    fn new(s: &[u8]) -> Self {
        let n = s.len();
        let mut tree = vec![Node::default(); 4 * n];
        Self::build(&mut tree, s, 1, 0, n - 1);
        Self { tree }
    }

    fn build(tree: &mut [Node], s: &[u8], node: usize, l: usize, r: usize) {
        if l == r {
            tree[node] = Node::new(s[l]);
            return;
        }
        let mid = l + (r - l) / 2;
        Self::build(tree, s, node * 2, l, mid);
        Self::build(tree, s, node * 2 + 1, mid + 1, r);
        tree[node] = Node::merge(&tree[node * 2], &tree[node * 2 + 1]);
    }

    fn update(&mut self, node: usize, l: usize, r: usize, idx: usize, c: u8) {
        if l == r {
            self.tree[node] = Node::new(c);
            return;
        }
        let mid = l + (r - l) / 2;
        if idx <= mid {
            self.update(node * 2, l, mid, idx, c);
        } else {
            self.update(node * 2 + 1, mid + 1, r, idx, c);
        }
        self.tree[node] = Node::merge(&self.tree[node * 2], &self.tree[node * 2 + 1]);
    }
}

impl Solution {
    pub fn longest_repeating(
        s: String,
        query_characters: String,
        query_indices: Vec<i32>,
    ) -> Vec<i32> {
        let n = s.len();
        let mut st = SegTree::new(s.as_bytes());
        let mut res = Vec::with_capacity(query_indices.len());

        let q_chars = query_characters.as_bytes();
        for (i, &idx) in query_indices.iter().enumerate() {
            st.update(1, 0, n - 1, idx as usize, q_chars[i]);
            res.push(st.tree[1].max_len);
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::longest_substring_of_one_repeating_character::Solution;

    #[test]
    fn test_longest_repeating_1() {
        let s = "babacc".to_string();
        let query_characters = "bcb".to_string();
        let query_indices = [1, 3, 3].to_vec();
        let output = [3, 3, 4].to_vec();
        assert_eq!(
            output,
            Solution::longest_repeating(s, query_characters, query_indices)
        );
    }

    #[test]
    fn test_longest_repeating_2() {
        let s = "abyzz".to_string();
        let query_characters = "aa".to_string();
        let query_indices = [2, 1].to_vec();
        let output = [2, 3].to_vec();
        assert_eq!(
            output,
            Solution::longest_repeating(s, query_characters, query_indices)
        );
    }
}
