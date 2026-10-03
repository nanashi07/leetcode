// 32. Longest Valid Parentheses
// https://leetcode.com/problems/longest-valid-parentheses/
struct Solution;

impl Solution {
    /// Scan once, counting the run of pairs opened by `opener`. A surplus of the matching
    /// closer ends the run; equal counts mark a valid substring of length `2 * opened`.
    fn scan<'a>(bytes: impl Iterator<Item = &'a u8>, opener: u8) -> usize {
        let (mut opened, mut closed, mut best) = (0_usize, 0_usize, 0_usize);
        for &byte in bytes {
            if byte == opener {
                opened += 1;
            } else {
                closed += 1;
            }
            if opened < closed {
                opened = 0;
                closed = 0;
            } else if opened == closed {
                best = best.max(opened * 2);
            }
        }
        best
    }

    pub fn longest_valid_parentheses(s: String) -> i32 {
        // A run of pairs can only be broken by an unmatched bracket. Scanning left to right
        // restarts on a surplus of ')' but never notices trailing '(', while the mirrored
        // right-to-left scan restarts on a surplus of '('; every longest valid substring is
        // measured by one of them. O(n) time, O(1) space (no stack needed).
        let bytes = s.as_bytes();
        Self::scan(bytes.iter(), b'(').max(Self::scan(bytes.iter().rev(), b')')) as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::longest_valid_parentheses::Solution;

    #[test]
    fn test_longest_valid_parentheses_1() {
        let s = "(()".to_string();
        assert_eq!(2, Solution::longest_valid_parentheses(s));
    }

    #[test]
    fn test_longest_valid_parentheses_2() {
        let s = ")()())".to_string();
        assert_eq!(4, Solution::longest_valid_parentheses(s));
    }

    #[test]
    fn test_longest_valid_parentheses_3() {
        let s = "".to_string();
        assert_eq!(0, Solution::longest_valid_parentheses(s));
    }
}
