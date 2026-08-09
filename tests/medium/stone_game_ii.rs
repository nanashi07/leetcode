// 1140. Stone Game II
// https://leetcode.com/problems/stone-game-ii/

struct Solution;

impl Solution {
    pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::stone_game_ii::Solution;

    #[test]
    fn test_stone_game_ii_1() {
        let piles = [2, 7, 9, 4, 4].to_vec();
        assert_eq!(10, Solution::stone_game_ii(piles));
    }

    #[test]
    fn test_stone_game_ii_2() {
        let piles = [1, 2, 3, 4, 5, 100].to_vec();
        assert_eq!(104, Solution::stone_game_ii(piles));
    }
}
