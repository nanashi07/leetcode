// 712. Minimum ASCII Delete Sum for Two Strings
// https://leetcode.com/problems/minimum-ascii-delete-sum-for-two-strings/

struct Solution;

impl Solution {
    pub fn minimum_delete_sum(s1: String, s2: String) -> i32 {
        let s1_bytes = s1.as_bytes();
        let s2_bytes = s2.as_bytes();
        let m = s1_bytes.len();
        let n = s2_bytes.len();

        // dp[i][j] represents the minimum delete sum to make s1[0..i] and s2[0..j] equal
        let mut dp = vec![vec![0; n + 1]; m + 1];

        // Base case: if s2 is empty, delete all characters from s1
        for i in 1..=m {
            dp[i][0] = dp[i - 1][0] + s1_bytes[i - 1] as i32;
        }

        // Base case: if s1 is empty, delete all characters from s2
        for j in 1..=n {
            dp[0][j] = dp[0][j - 1] + s2_bytes[j - 1] as i32;
        }

        // Fill the dp table
        for i in 1..=m {
            for j in 1..=n {
                if s1_bytes[i - 1] == s2_bytes[j - 1] {
                    // Characters match, no deletion needed
                    dp[i][j] = dp[i - 1][j - 1];
                } else {
                    // Characters don't match, delete from either s1 or s2
                    dp[i][j] = std::cmp::min(
                        dp[i - 1][j] + s1_bytes[i - 1] as i32, // Delete from s1
                        dp[i][j - 1] + s2_bytes[j - 1] as i32, // Delete from s2
                    );
                }
            }
        }

        dp[m][n]
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::minimum_ascii_delete_sum_for_two_strings::Solution;

    #[test]
    fn test_minimum_delete_sum_1() {
        let s1 = "sea".to_string();
        let s2 = "eat".to_string();
        assert_eq!(231, Solution::minimum_delete_sum(s1, s2));
    }

    #[test]
    fn test_minimum_delete_sum_2() {
        let s1 = "delete".to_string();
        let s2 = "leet".to_string();
        assert_eq!(403, Solution::minimum_delete_sum(s1, s2));
    }
}
