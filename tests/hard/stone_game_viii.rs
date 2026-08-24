// 1872. Stone Game VIII
// https://leetcode.com/problems/stone-game-viii/

struct Solution;

impl Solution {
    pub fn stone_game_viii(stones: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::stone_game_viii::Solution;

    #[test]
    fn test_stone_game_viii_1() {
        let stones = [-1, 2, -3, 4, -5].to_vec();
        assert_eq!(5, Solution::stone_game_viii(stones));
    }

    #[test]
    fn test_stone_game_viii_2() {
        let stones = [7, -6, 5, 10, 5, -2, -6].to_vec();
        assert_eq!(13, Solution::stone_game_viii(stones));
    }

    #[test]
    fn test_stone_game_viii_3() {
        let stones = [-10, -12].to_vec();
        assert_eq!(-22, Solution::stone_game_viii(stones));
    }
}
