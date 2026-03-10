// 3130. Find All Possible Stable Binary Arrays II
// https://leetcode.com/problems/find-all-possible-stable-binary-arrays-ii/

struct Solution;

impl Solution {
    pub fn number_of_stable_arrays(zero: i32, one: i32, limit: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let z = zero as usize;
        let o = one as usize;
        let lim = limit as usize;

        // dp0[i][j] = # stable arrays with i zeros and j ones, ending with 0
        // dp1[i][j] = # stable arrays with i zeros and j ones, ending with 1
        let mut dp0 = vec![vec![0i64; o + 1]; z + 1];
        let mut dp1 = vec![vec![0i64; o + 1]; z + 1];

        // prefix1[i][j] = sum_{k=0}^{i} dp1[k][j]  (prefix over rows, for dp0 transitions)
        // prefix0[i][j] = sum_{k=0}^{j} dp0[i][k]  (prefix over cols, for dp1 transitions)
        let mut prefix1 = vec![vec![0i64; o + 1]; z + 1];
        let mut prefix0 = vec![vec![0i64; o + 1]; z + 1];

        for i in 0..=z {
            for j in 0..=o {
                // Compute dp0[i][j]
                if j == 0 {
                    // All-zeros array: valid only if 1 <= i <= limit
                    dp0[i][0] = if i >= 1 && i <= lim { 1 } else { 0 };
                } else if i >= 1 {
                    // Last run of 0s has length t (1..=min(i,lim)), preceded by a 1
                    // dp0[i][j] = sum_{t=1}^{min(i,lim)} dp1[i-t][j]
                    //           = prefix1[i-1][j] - prefix1[i-lim-1][j]  (clamped)
                    let high = i - 1;
                    let low = i.saturating_sub(lim); // i - min(i,lim), same as max(0, i-lim)
                    dp0[i][j] = prefix1[high][j];
                    if low > 0 {
                        dp0[i][j] = (dp0[i][j] - prefix1[low - 1][j] + MOD) % MOD;
                    }
                }

                // Compute dp1[i][j]
                if i == 0 {
                    // All-ones array: valid only if 1 <= j <= limit
                    dp1[0][j] = if j >= 1 && j <= lim { 1 } else { 0 };
                } else if j >= 1 {
                    // Last run of 1s has length t (1..=min(j,lim)), preceded by a 0
                    // dp1[i][j] = sum_{t=1}^{min(j,lim)} dp0[i][j-t]
                    //           = prefix0[i][j-1] - prefix0[i][j-lim-1]  (clamped)
                    let high = j - 1;
                    let low = j.saturating_sub(lim);
                    dp1[i][j] = prefix0[i][high];
                    if low > 0 {
                        dp1[i][j] = (dp1[i][j] - prefix0[i][low - 1] + MOD) % MOD;
                    }
                }

                // Update prefix sums
                prefix1[i][j] = (dp1[i][j] + if i > 0 { prefix1[i - 1][j] } else { 0 }) % MOD;
                prefix0[i][j] = (dp0[i][j] + if j > 0 { prefix0[i][j - 1] } else { 0 }) % MOD;
            }
        }

        ((dp0[z][o] + dp1[z][o]) % MOD) as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::find_all_possible_stable_binary_arrays_ii::Solution;

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
