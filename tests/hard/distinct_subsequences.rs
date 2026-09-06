// 115. Distinct Subsequences
// https://leetcode.com/problems/distinct-subsequences/

struct Solution;

impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let (s, t) = (s.as_bytes(), t.as_bytes());
        let mut dp = vec![0i64; t.len() + 1];
        dp[0] = 1;
        for &sc in s {
            for j in (1..=t.len()).rev() {
                if sc == t[j - 1] {
                    dp[j] += dp[j - 1];
                }
            }
        }
        dp[t.len()] as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::distinct_subsequences::Solution;

    #[test]
    fn test_num_distinct_1() {
        let s = "rabbbit".to_string();
        let t = "rabbit".to_string();
        assert_eq!(3, Solution::num_distinct(s, t));
    }

    #[test]
    fn test_num_distinct_2() {
        let s = "babgbag".to_string();
        let t = "bag".to_string();
        assert_eq!(5, Solution::num_distinct(s, t));
    }
}
