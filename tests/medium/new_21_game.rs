// # 837. New 21 Game
// https://leetcode.com/problems/new-21-game/

struct Solution;

impl Solution {
    pub fn new21_game(n: i32, k: i32, max_pts: i32) -> f64 {
        let n = n as usize;
        let k = k as usize;
        let max_pts = max_pts as usize;

        let mut dp = vec![0.0; n + 1];
        dp[0] = 1.0;
        let mut s = if k > 0 { 1.0 } else { 0.0 };

        for i in 1..=n {
            dp[i] = s / max_pts as f64;

            if i < k {
                s += dp[i];
            }

            if i >= max_pts && i - max_pts < k {
                s -= dp[i - max_pts];
            }
        }

        let mut ans = 0.0;
        for i in k..=n {
            ans += dp[i];
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::new_21_game::Solution;

    const D: f64 = 100000.0;

    #[test]
    fn test_new21_game_1() {
        let n = 10;
        let k = 1;
        let max_pts = 10;
        assert_eq!(
            1.00000,
            (Solution::new21_game(n, k, max_pts) * D).round() / D
        );
    }

    #[test]
    fn test_new21_game_2() {
        let n = 6;
        let k = 1;
        let max_pts = 10;
        assert_eq!(
            0.60000,
            (Solution::new21_game(n, k, max_pts) * D).round() / D
        );
    }

    #[test]
    fn test_new21_game_3() {
        let n = 21;
        let k = 17;
        let max_pts = 10;
        assert_eq!(
            0.73278,
            (Solution::new21_game(n, k, max_pts) * D).round() / D
        );
    }
}
