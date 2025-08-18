// # 679. 24 Game
// https://leetcode.com/problems/24-game/

struct Solution;

impl Solution {
    pub fn judge_point24(cards: Vec<i32>) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::twenty_four_game::Solution;

    #[test]
    fn test_judge_point24_1() {
        let cards = [4, 1, 8, 7].to_vec();
        assert_eq!(true, Solution::judge_point24(cards));
    }

    #[test]
    fn test_judge_point24_2() {
        let cards = [1, 2, 1, 2].to_vec();
        assert_eq!(false, Solution::judge_point24(cards));
    }
}
