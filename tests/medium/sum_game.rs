// 1927. Sum Game
// https://leetcode.com/problems/sum-game/

struct Solution;

impl Solution {
    pub fn sum_game(num: String) -> bool {
        let mid = num.len() / 2;
        let mut diff = 0;
        let mut dq = 0;

        for (i, &c) in num.as_bytes().iter().enumerate() {
            if i < mid {
                if c == b'?' {
                    dq -= 1;
                } else {
                    diff += (c - b'0') as i32;
                }
            } else {
                if c == b'?' {
                    dq += 1;
                } else {
                    diff -= (c - b'0') as i32;
                }
            }
        }

        diff * 2 != dq * 9
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::sum_game::Solution;

    #[test]
    fn test_sum_game_1() {
        let num = "5023".to_string();
        assert!(!Solution::sum_game(num));
    }

    #[test]
    fn test_sum_game_2() {
        let num = "25??".to_string();
        assert!(Solution::sum_game(num));
    }

    #[test]
    fn test_sum_game_3() {
        let num = "?3295???".to_string();
        assert!(!Solution::sum_game(num));
    }
}
