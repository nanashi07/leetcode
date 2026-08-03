// 1406. Stone Game III
// https://leetcode.com/problems/stone-game-iii/

struct Solution;

impl Solution {
    pub fn stone_game_iii(stone_value: Vec<i32>) -> String {
        let n = stone_value.len();
        let mut dp = vec![i32::MIN; n + 1];
        dp[n] = 0;
        let suffix: Vec<i32> = {
            let mut s = vec![0; n + 1];
            for i in (0..n).rev() {
                s[i] = s[i + 1] + stone_value[i];
            }
            s
        };
        for i in (0..n).rev() {
            for k in 1..=3 {
                if i + k <= n {
                    let take = suffix[i] - suffix[i + k];
                    dp[i] = dp[i].max(take - dp[i + k]);
                }
            }
        }
        match dp[0].cmp(&0) {
            std::cmp::Ordering::Greater => "Alice".to_string(),
            std::cmp::Ordering::Less => "Bob".to_string(),
            std::cmp::Ordering::Equal => "Tie".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::stone_game_iii::Solution;

    #[test]
    fn test_stone_game_iii_1() {
        let stone_value = [1, 2, 3, 7].to_vec();
        assert_eq!("Bob".to_string(), Solution::stone_game_iii(stone_value));
    }

    #[test]
    fn test_stone_game_iii_2() {
        let stone_value = [1, 2, 3, -9].to_vec();
        assert_eq!("Alice".to_string(), Solution::stone_game_iii(stone_value));
    }

    #[test]
    fn test_stone_game_iii_3() {
        let stone_value = [1, 2, 3, 6].to_vec();
        assert_eq!("Tie".to_string(), Solution::stone_game_iii(stone_value));
    }
}
