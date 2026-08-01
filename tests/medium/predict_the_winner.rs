// 486. Predict the Winner
// https://leetcode.com/problems/predict-the-winner/

struct Solution;

impl Solution {
    pub fn predict_the_winner(nums: Vec<i32>) -> bool {
        let n = nums.len();
        if n & 1 == 0 {
            return true;
        }
        let mut dp = nums.clone();
        for i in (0..n).rev() {
            for j in (i + 1)..n {
                dp[j] = std::cmp::max(nums[i] - dp[j], nums[j] - dp[j - 1]);
            }
        }
        dp[n - 1] >= 0
    }
}

#[cfg(test)]
#[allow(clippy::bool_assert_comparison)]
mod tests {
    use crate::medium::predict_the_winner::Solution;

    #[test]
    fn test_predict_the_winner_1() {
        let nums = [1, 5, 2].to_vec();
        assert_eq!(false, Solution::predict_the_winner(nums));
    }

    #[test]
    fn test_predict_the_winner_2() {
        let nums = [1, 5, 233, 7].to_vec();
        assert_eq!(true, Solution::predict_the_winner(nums));
    }
}
