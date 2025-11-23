// 1262. Greatest Sum Divisible by Three
// https://leetcode.com/problems/greatest-sum-divisible-by-three/

struct Solution;

impl Solution {
    pub fn max_sum_div_three(nums: Vec<i32>) -> i32 {
        // dp[i] = maximum sum with remainder i when divided by 3
        // dp[0] = sum divisible by 3
        // dp[1] = sum with remainder 1
        // dp[2] = sum with remainder 2
        let mut dp = [0, i32::MIN, i32::MIN];

        for num in nums {
            let mut temp = dp;
            let remainder = (num % 3) as usize;

            // Try adding current number to each previous state
            for i in 0..3 {
                if dp[i] != i32::MIN {
                    let new_remainder = (i + remainder) % 3;
                    temp[new_remainder] = temp[new_remainder].max(dp[i] + num);
                }
            }

            dp = temp;
        }

        dp[0]
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::greatest_sum_divisible_by_three::Solution;

    #[test]
    fn test_max_sum_div_three_1() {
        let nums = [3, 6, 5, 1, 8].to_vec();
        assert_eq!(18, Solution::max_sum_div_three(nums));
    }

    #[test]
    fn test_max_sum_div_three_2() {
        let nums = [4].to_vec();
        assert_eq!(0, Solution::max_sum_div_three(nums));
    }

    #[test]
    fn test_max_sum_div_three_3() {
        let nums = [1, 2, 3, 4, 4].to_vec();
        assert_eq!(12, Solution::max_sum_div_three(nums));
    }
}
