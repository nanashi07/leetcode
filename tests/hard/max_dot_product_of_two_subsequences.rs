// 1458. Max Dot Product of Two Subsequences
// https://leetcode.com/problems/max-dot-product-of-two-subsequences/

struct Solution;

impl Solution {
    pub fn max_dot_product(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let n = nums1.len();
        let m = nums2.len();

        // dp[i][j] represents the maximum dot product using elements up to index i in nums1
        // and up to index j in nums2
        let mut dp = vec![vec![i32::MIN / 2; m]; n];

        // Base case: first element
        dp[0][0] = nums1[0] * nums2[0];

        // Fill first row: only using nums1[0] with elements from nums2
        for j in 1..m {
            dp[0][j] = dp[0][j - 1].max(nums1[0] * nums2[j]);
        }

        // Fill first column: only using nums2[0] with elements from nums1
        for i in 1..n {
            dp[i][0] = dp[i - 1][0].max(nums1[i] * nums2[0]);
        }

        // Fill the rest of the dp table
        for i in 1..n {
            for j in 1..m {
                let product = nums1[i] * nums2[j];

                // Four options:
                // 1. Don't include nums1[i] - use dp[i-1][j]
                // 2. Don't include nums2[j] - use dp[i][j-1]
                // 3. Include both nums1[i] and nums2[j] as a new subsequence start
                // 4. Include both nums1[i] and nums2[j] extending previous subsequence
                dp[i][j] = dp[i - 1][j]
                    .max(dp[i][j - 1])
                    .max(product)
                    .max(dp[i - 1][j - 1] + product);
            }
        }

        dp[n - 1][m - 1]
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::max_dot_product_of_two_subsequences::Solution;

    #[test]
    fn test_max_dot_product_1() {
        let nums1 = [2, 1, -2, 5].to_vec();
        let nums2 = [3, 0, -6].to_vec();
        assert_eq!(18, Solution::max_dot_product(nums1, nums2));
    }

    #[test]
    fn test_max_dot_product_2() {
        let nums1 = [3, -2].to_vec();
        let nums2 = [2, -6, 7].to_vec();
        assert_eq!(21, Solution::max_dot_product(nums1, nums2));
    }

    #[test]
    fn test_max_dot_product_3() {
        let nums1 = [-1, -1].to_vec();
        let nums2 = [1, 1].to_vec();
        assert_eq!(-1, Solution::max_dot_product(nums1, nums2));
    }
}
