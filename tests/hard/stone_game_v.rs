// 1563. Stone Game V
// https://leetcode.com/problems/stone-game-v/

struct Solution;

impl Solution {
    pub fn stone_game_v(stone_value: Vec<i32>) -> i32 {
        let n = stone_value.len();
        if n < 2 {
            return 0;
        }

        let mut prefix = vec![0; n + 1];
        for (i, &value) in stone_value.iter().enumerate() {
            prefix[i + 1] = prefix[i] + value;
        }

        let mut dp = vec![vec![0; n]; n];
        for length in 2..=n {
            for left in 0..=n - length {
                let right = left + length - 1;
                for split in left..right {
                    let left_sum = prefix[split + 1] - prefix[left];
                    let right_sum = prefix[right + 1] - prefix[split + 1];
                    let score = if left_sum <= right_sum {
                        left_sum + dp[left][split]
                    } else {
                        right_sum + dp[split + 1][right]
                    };
                    dp[left][right] = dp[left][right].max(score);
                }
            }
        }

        dp[0][n - 1]
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::stone_game_v::Solution;

    #[test]
    fn test_stone_game_v_1() {
        let stone_value = [6, 2, 3, 4, 5, 5].to_vec();
        assert_eq!(28, Solution::stone_game_v(stone_value));
    }

    #[test]
    fn test_stone_game_v_2() {
        let stone_value = [4].to_vec();
        assert_eq!(0, Solution::stone_game_v(stone_value));
    }
}
