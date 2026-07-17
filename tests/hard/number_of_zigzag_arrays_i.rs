// 3699. Number of ZigZag Arrays I
// https://leetcode.com/problems/number-of-zigzag-arrays-i/

struct Solution;

impl Solution {
    #[allow(clippy::needless_range_loop)]
    pub fn zig_zag_arrays(n: i32, l: i32, r: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let n = n as usize;
        let l = l as usize;
        let r = r as usize;
        let m = r - l + 1;
        if n == 1 {
            return (m % MOD as usize) as i32;
        }
        // dp[v][0] = ways ending with value (l+v) needing next to go up
        // dp[v][1] = ways ending with value (l+v) needing next to go down
        let mut dp = vec![[0i64; 2]; m];
        for v in 0..m {
            dp[v][0] = 1; // will go up next (started high, came down)
            dp[v][1] = 1; // will go down next (started low, went up)
        }
        for _ in 1..n {
            let mut ndp = vec![[0i64; 2]; m];
            // prefix sum for transitions
            // dir=0 (need up): next value must be > current, next direction = 1
            // dir=1 (need down): next value must be < current, next direction = 0
            let mut sum_up = vec![0i64; m + 1]; // sum_up[i] = sum of dp[0..i][0]
            for v in 0..m {
                sum_up[v + 1] = (sum_up[v] + dp[v][0]) % MOD;
            }
            let mut sum_down = vec![0i64; m + 1]; // sum_down[i] = sum of dp[0..i][1]
            for v in 0..m {
                sum_down[v + 1] = (sum_down[v] + dp[v][1]) % MOD;
            }
            for v in 0..m {
                // v is the new value; it was reached by going up from some u < v
                // those u had dir=0 (needing up), now v's dir=1 (needing down)
                ndp[v][1] = sum_up[v]; // sum of dp[0..v][0]
                                       // v was reached by going down from some u > v
                                       // those u had dir=1 (needing down), now v's dir=0 (needing up)
                ndp[v][0] = (sum_down[m] - sum_down[v + 1] + MOD) % MOD;
            }
            dp = ndp;
        }
        let mut ans = 0i64;
        for (v, _) in dp.iter().take(m).enumerate() {
            ans = (ans + dp[v][0] + dp[v][1]) % MOD;
        }
        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::number_of_zigzag_arrays_i::Solution;

    #[test]
    fn test_zig_zag_arrays_1() {
        let n = 3;
        let l = 4;
        let r = 5;
        assert_eq!(2, Solution::zig_zag_arrays(n, l, r));
    }

    #[test]
    fn test_zig_zag_arrays_2() {
        let n = 3;
        let l = 1;
        let r = 3;
        assert_eq!(10, Solution::zig_zag_arrays(n, l, r));
    }
}
