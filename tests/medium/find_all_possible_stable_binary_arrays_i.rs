// 3129. Find All Possible Stable Binary Arrays I
// https://leetcode.com/problems/find-all-possible-stable-binary-arrays-i/

struct Solution;

impl Solution {
    pub fn number_of_stable_arrays(zero: i32, one: i32, limit: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let z = zero as usize;
        let o = one as usize;
        let l = limit as usize;

        // dp[i][j][k]: # stable arrays with i zeros, j ones, ending with k (0 or 1)
        let mut dp = vec![vec![[0i64; 2]; o + 1]; z + 1];

        // Base: only zeros or only ones
        for i in 1..=z.min(l) {
            dp[i][0][0] = 1;
        }
        for j in 1..=o.min(l) {
            dp[0][j][1] = 1;
        }

        for i in 1..=z {
            for j in 1..=o {
                // Ending with 0: last run of t zeros (1..=min(i,l)) preceded by a 1
                for t in 1..=i.min(l) {
                    dp[i][j][0] = (dp[i][j][0] + dp[i - t][j][1]) % MOD;
                }
                // Ending with 1: last run of t ones (1..=min(j,l)) preceded by a 0
                for t in 1..=j.min(l) {
                    dp[i][j][1] = (dp[i][j][1] + dp[i][j - t][0]) % MOD;
                }
            }
        }

        ((dp[z][o][0] + dp[z][o][1]) % MOD) as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::find_all_possible_stable_binary_arrays_i::Solution;

    #[test]
    fn test_number_of_stable_arrays_1() {
        let zero = 1;
        let one = 1;
        let limit = 2;
        assert_eq!(2, Solution::number_of_stable_arrays(zero, one, limit));
    }

    #[test]
    fn test_number_of_stable_arrays_2() {
        let zero = 1;
        let one = 2;
        let limit = 1;
        assert_eq!(1, Solution::number_of_stable_arrays(zero, one, limit));
    }

    #[test]
    fn test_number_of_stable_arrays_3() {
        let zero = 3;
        let one = 3;
        let limit = 2;
        assert_eq!(14, Solution::number_of_stable_arrays(zero, one, limit));
    }
}
