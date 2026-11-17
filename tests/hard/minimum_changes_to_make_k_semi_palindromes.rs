// 2911. Minimum Changes to Make K Semi-palindromes
// https://leetcode.com/problems/minimum-changes-to-make-k-semi-palindromes/

struct Solution;

impl Solution {
    /// A substring of length `len >= 2` becomes a semi-palindrome by picking a proper
    /// divisor `d` of `len`; the `d` stride groups (offsets `0..d`) must each read the same
    /// forwards and backwards, so the cheapest fix for that `d` is one change per mismatching
    /// symmetric pair. Length 1 has no proper divisor, hence it can never be a semi-palindrome.
    const INF: i32 = 1 << 20;

    pub fn minimum_changes(s: String, k: i32) -> i32 {
        let bytes = s.as_bytes();
        let n = bytes.len();
        let k = k as usize;

        // cost[l][r] = minimum changes to turn s[l..=r] into a semi-palindrome.
        // For divisor d the work is len/(2d) pair comparisons, so summing over divisors of
        // every substring stays around O(n^3 / 3) with a tiny constant (n <= 200).
        let mut cost = vec![vec![Self::INF; n]; n];
        for l in 0..n {
            let row = &mut cost[l];
            for (r, cell) in row.iter_mut().enumerate().skip(l + 1) {
                let len = r - l + 1;
                let mut best = Self::INF;
                for d in 1..len {
                    if len % d != 0 {
                        continue;
                    }
                    // Group size along the stride; each group is reversed onto itself.
                    let last = len / d - 1;
                    let mut changes = 0;
                    for offset in 0..d {
                        let (mut a, mut b) = (offset, last * d + offset);
                        while a < b {
                            if bytes[l + a] != bytes[l + b] {
                                changes += 1;
                            }
                            a += d;
                            b -= d;
                        }
                    }
                    if changes < best {
                        best = changes;
                    }
                }
                *cell = best;
            }
        }

        // dp[i] after `part` rounds = cheapest split of s[..i] into `part` semi-palindromes.
        // Every part needs length >= 2, so a prefix shorter than 2 * part is unreachable.
        // One-part case: the whole prefix is a single semi-palindrome.
        let mut prev = vec![Self::INF; n + 1];
        if n > 1 {
            prev[2..=n].copy_from_slice(&cost[0][1..n]);
        }

        for part in 2..=k {
            let mut cur = vec![Self::INF; n + 1];
            // Leave at least two characters for each of the remaining parts; an empty
            // range simply means this split is impossible and stays unreachable.
            let hi = n.saturating_sub(2 * (k - part));
            for i in 2 * part..=hi {
                let mut best = Self::INF;
                for cut in 2 * (part - 1)..=i - 2 {
                    if prev[cut] == Self::INF {
                        continue;
                    }
                    let total = prev[cut] + cost[cut][i - 1];
                    if total < best {
                        best = total;
                    }
                }
                cur[i] = best;
            }
            prev = cur;
        }

        prev[n]
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::minimum_changes_to_make_k_semi_palindromes::Solution;

    #[test]
    fn test_minimum_changes_1() {
        let s: String = "abcac".to_string();
        let k: i32 = 2;
        assert_eq!(1, Solution::minimum_changes(s, k));
    }

    #[test]
    fn test_minimum_changes_2() {
        let s: String = "abcac".to_string();
        let k: i32 = 2;
        assert_eq!(1, Solution::minimum_changes(s, k));
    }

    #[test]
    fn test_minimum_changes_3() {
        let s: String = "abcac".to_string();
        let k: i32 = 2;
        assert_eq!(1, Solution::minimum_changes(s, k));
    }
}
