// 3093. Longest Common Suffix Queries
// https://leetcode.com/problems/longest-common-suffix-queries/

struct Solution;

#[derive(Clone, Copy)]
struct Best {
    len: usize,
    idx: usize,
}

impl Best {
    fn better_than(self, other: Self) -> bool {
        self.len < other.len || (self.len == other.len && self.idx < other.idx)
    }
}

struct Node {
    next: [Option<usize>; 26],
    best: Best,
}

impl Node {
    fn new(best: Best) -> Self {
        Self {
            next: [None; 26],
            best,
        }
    }
}

impl Solution {
    pub fn string_indices(words_container: Vec<String>, words_query: Vec<String>) -> Vec<i32> {
        let initial_best = Best {
            len: usize::MAX,
            idx: usize::MAX,
        };
        let mut trie = vec![Node::new(initial_best)];

        for (i, w) in words_container.iter().enumerate() {
            let candidate = Best {
                len: w.len(),
                idx: i,
            };

            if candidate.better_than(trie[0].best) {
                trie[0].best = candidate;
            }

            let mut node = 0usize;
            for &b in w.as_bytes().iter().rev() {
                let c = (b - b'a') as usize;
                if trie[node].next[c].is_none() {
                    trie.push(Node::new(initial_best));
                    let nxt = trie.len() - 1;
                    trie[node].next[c] = Some(nxt);
                }
                node = trie[node].next[c].unwrap();
                if candidate.better_than(trie[node].best) {
                    trie[node].best = candidate;
                }
            }
        }

        let mut ans = Vec::with_capacity(words_query.len());
        for q in &words_query {
            let mut node = 0usize;
            for &b in q.as_bytes().iter().rev() {
                let c = (b - b'a') as usize;
                match trie[node].next[c] {
                    Some(nxt) => node = nxt,
                    None => break,
                }
            }
            ans.push(trie[node].best.idx as i32);
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::longest_common_suffix_queries::Solution;
    use crate::shared::vec2d::to_string_vec;

    #[test]
    fn test_string_indices_1() {
        let words_container = to_string_vec(["abcd", "bcd", "xbcd"]);
        let words_query = to_string_vec(["cd", "bcd", "xyz"]);
        assert_eq!(
            [1, 1, 1].to_vec(),
            Solution::string_indices(words_container, words_query)
        );
    }

    #[test]
    fn test_string_indices_2() {
        let words_container = to_string_vec(["abcdefgh", "poiuygh", "ghghgh"]);
        let words_query = to_string_vec(["gh", "acbfgh", "acbfegh"]);
        assert_eq!(
            [2, 0, 2].to_vec(),
            Solution::string_indices(words_container, words_query)
        );
    }
}
