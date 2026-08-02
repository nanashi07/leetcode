// 877. Stone Game
// https://leetcode.com/problems/stone-game/

struct Solution;

impl Solution {
    pub fn stone_game(piles: Vec<i32>) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::stone_game::Solution;

    #[test]
    fn test_stone_game_1() {
        let piles = [5, 3, 4, 5].to_vec();
        assert_eq!(true, Solution::stone_game(piles));
    }

    #[test]
    fn test_stone_game_2() {
        let piles = [3, 7, 2, 3].to_vec();
        assert_eq!(true, Solution::stone_game(piles));
    }
}
