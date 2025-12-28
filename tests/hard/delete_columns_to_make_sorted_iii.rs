// 960. Delete Columns to Make Sorted III
// https://leetcode.com/problems/delete-columns-to-make-sorted-iii/

struct Solution;

impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        if strs.is_empty() {
            return 0;
        }

        let n = strs.len();
        let m = strs[0].len();

        if m == 0 {
            return 0;
        }

        // Convert strings to vectors of bytes for easier access
        let strs_bytes: Vec<Vec<u8>> = strs.iter().map(|s| s.as_bytes().to_vec()).collect();

        // dp[i] = maximum number of columns we can keep ending at column i
        let mut dp = vec![1; m];

        // For each column i
        for i in 0..m {
            // Check all previous columns j
            for j in 0..i {
                // Check if column j can come before column i
                // (all characters in column j should be <= corresponding characters in column i)
                let mut valid = true;
                for row in 0..n {
                    if strs_bytes[row][j] > strs_bytes[row][i] {
                        valid = false;
                        break;
                    }
                }

                if valid {
                    dp[i] = dp[i].max(dp[j] + 1);
                }
            }
        }

        // Find the maximum number of columns we can keep
        let max_keep = *dp.iter().max().unwrap();

        // Return the minimum number of deletions
        (m - max_keep) as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::delete_columns_to_make_sorted_iii::Solution;

    #[test]
    fn test_min_deletion_size_1() {
        let strs = ["babca", "bbazb"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        assert_eq!(3, Solution::min_deletion_size(strs));
    }

    #[test]
    fn test_min_deletion_size_2() {
        let strs = ["edcba"].iter().map(|s| s.to_string()).collect::<Vec<_>>();
        assert_eq!(4, Solution::min_deletion_size(strs));
    }

    #[test]
    fn test_min_deletion_size_3() {
        let strs = ["ghi", "def", "abc"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        assert_eq!(0, Solution::min_deletion_size(strs));
    }
}
