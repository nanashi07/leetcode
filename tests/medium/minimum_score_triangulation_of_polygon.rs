// 1039. Minimum Score Triangulation of Polygon
// https://leetcode.com/problems/minimum-score-triangulation-of-polygon/

struct Solution;

impl Solution {
    pub fn min_score_triangulation(values: Vec<i32>) -> i32 {
        let n = values.len();
        // dp[i][j] represents the minimum score to triangulate polygon from vertex i to j
        let mut dp = vec![vec![0; n]; n];

        // len is the length of the chain (number of vertices in the sub-polygon)
        // We start from length 3 (minimum for a triangle)
        for len in 3..=n {
            // i is the starting vertex
            for i in 0..=n - len {
                let j = i + len - 1; // ending vertex
                dp[i][j] = i32::MAX;

                // Try all possible middle vertices k to form triangle (i, k, j)
                for k in i + 1..j {
                    // Score of triangle (i, k, j) + score of left sub-polygon + score of right sub-polygon
                    let score = values[i] * values[k] * values[j] + dp[i][k] + dp[k][j];
                    dp[i][j] = dp[i][j].min(score);
                }
            }
        }

        // Return the minimum score for the entire polygon (from vertex 0 to n-1)
        dp[0][n - 1]
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::minimum_score_triangulation_of_polygon::Solution;

    #[test]
    fn test_min_score_triangulation_1() {
        let values = [1, 2, 3].to_vec();
        assert_eq!(6, Solution::min_score_triangulation(values));
    }

    #[test]
    fn test_min_score_triangulation_2() {
        let values = [3, 7, 4, 5].to_vec();
        assert_eq!(144, Solution::min_score_triangulation(values));
    }

    #[test]
    fn test_min_score_triangulation_3() {
        let values = [1, 3, 1, 4, 1, 5].to_vec();
        assert_eq!(13, Solution::min_score_triangulation(values));
    }
}
