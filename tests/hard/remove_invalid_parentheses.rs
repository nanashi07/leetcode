// 301. Remove Invalid Parentheses
// https://leetcode.com/problems/remove-invalid-parentheses/

struct Solution;

/// Search state for `Solution::remove_invalid_parentheses`.
struct Search<'a> {
    bytes: &'a [u8],
    /// `brackets[i]` is the number of `'('` and `')'` available in `bytes[i..]`.
    brackets: Vec<(usize, usize)>,
    /// Kept bytes of the branch currently being explored.
    path: Vec<u8>,
    result: Vec<String>,
}

impl Search<'_> {
    /// Explore `bytes[i..]` with `drop_open` / `drop_close` brackets still to remove,
    /// `open` unmatched kept `'('`, and whether `bytes[i - 1]` was removed.
    fn search(
        &mut self,
        i: usize,
        prev_dropped: bool,
        drop_open: usize,
        drop_close: usize,
        open: usize,
    ) {
        // Not enough brackets left to spend the removal budget or to close what is open.
        if drop_open > self.brackets[i].0 || open + drop_close > self.brackets[i].1 {
            return;
        }

        // The budget is exactly spent and nothing is left open: `path` is a shortest fix.
        if i == self.bytes.len() {
            // Only bracket bytes are ever skipped, so `path` stays valid UTF-8.
            let path = String::from_utf8(self.path.clone()).unwrap();
            self.result.push(path);
            return;
        }

        let b = self.bytes[i];
        let is_open = b == b'(';
        let is_close = b == b')';

        // Drop it. Only allowed at the head of a run of identical brackets, so each set of
        // removed positions is reached once and no de-duplication set is needed.
        let can_drop = (is_open && drop_open > 0) || (is_close && drop_close > 0);
        if can_drop && (prev_dropped || i == 0 || self.bytes[i - 1] != b) {
            self.search(
                i + 1,
                true,
                drop_open - usize::from(is_open),
                drop_close - usize::from(is_close),
                open,
            );
        }

        // Keep it. A ')' needs an already kept '(', because later removals cannot help it.
        if !is_close || open > 0 {
            self.path.push(b);
            let open = if is_open {
                open + 1
            } else if is_close {
                open - 1
            } else {
                open
            };
            self.search(i + 1, false, drop_open, drop_close, open);
            self.path.pop();
        }
    }
}

impl Solution {
    pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
        let bytes = s.into_bytes();
        let n = bytes.len();

        // Minimum number of '(' and ')' that have to be removed.
        let mut drop_open = 0usize;
        let mut drop_close = 0usize;
        for &b in &bytes {
            match b {
                b'(' => drop_open += 1,
                b')' => {
                    if drop_open == 0 {
                        drop_close += 1;
                    } else {
                        drop_open -= 1;
                    }
                }
                _ => {}
            }
        }

        let mut brackets = vec![(0usize, 0usize); n + 1];
        for i in (0..n).rev() {
            brackets[i] = brackets[i + 1];
            match bytes[i] {
                b'(' => brackets[i].0 += 1,
                b')' => brackets[i].1 += 1,
                _ => {}
            }
        }

        let mut search = Search {
            bytes: &bytes,
            brackets,
            path: Vec::with_capacity(n),
            result: Vec::new(),
        };
        search.search(0, false, drop_open, drop_close, 0);
        search.result
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::remove_invalid_parentheses::Solution;
    use crate::shared::vec2d::to_string_vec;

    #[test]
    fn test_remove_invalid_parentheses_1() {
        let s = "()())()".to_string();
        let output = to_string_vec(["(())()", "()()()"]);
        assert_eq!(output, Solution::remove_invalid_parentheses(s));
    }

    #[test]
    fn test_remove_invalid_parentheses_2() {
        let s = "(a)())()".to_string();
        let output = to_string_vec(["(a())()", "(a)()()"]);
        assert_eq!(output, Solution::remove_invalid_parentheses(s));
    }

    #[test]
    fn test_remove_invalid_parentheses_3() {
        let s = ")(".to_string();
        let output = to_string_vec([""]);
        assert_eq!(output, Solution::remove_invalid_parentheses(s));
    }
}
