// 1927. Sum Game
// https://leetcode.com/problems/sum-game/

struct Solution;

impl Solution {
    pub fn sum_game(num: String) -> bool {
        todo!()
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
