// 940. Distinct Subsequences II
// https://leetcode.com/problems/distinct-subsequences-ii/

struct Solution;

impl Solution {
    pub fn distinct_subseq_ii(s: String) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let mut dp = 1i64; // empty subsequence
        let mut last = [0i64; 26];
        for b in s.bytes() {
            let i = (b - b'a') as usize;
            let prev = last[i];
            last[i] = dp;
            dp = ((2 * dp - prev) % MOD + MOD) % MOD;
        }
        ((dp - 1 + MOD) % MOD) as i32 // subtract empty subsequence
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::distinct_subsequences_ii::Solution;

    #[test]
    fn test_distinct_subseq_ii_1() {
        let s = "abc".to_string();
        assert_eq!(7, Solution::distinct_subseq_ii(s));
    }

    #[test]
    fn test_distinct_subseq_ii_2() {
        let s = "aba".to_string();
        assert_eq!(6, Solution::distinct_subseq_ii(s));
    }

    #[test]
    fn test_distinct_subseq_ii_3() {
        let s = "aaa".to_string();
        assert_eq!(3, Solution::distinct_subseq_ii(s));
    }
}
