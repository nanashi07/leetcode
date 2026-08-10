// 1510. Stone Game IV
// https://leetcode.com/problems/stone-game-iv/

struct Solution;

impl Solution {
    pub fn winner_square_game(n: i32) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::stone_game_iv::Solution;

    #[test]
    fn test_winner_square_game_1() {
        let n = 1;
        assert_eq!(true, Solution::winner_square_game(n));
    }

    #[test]
    fn test_winner_square_game_2() {
        let n = 2;
        assert_eq!(false, Solution::winner_square_game(n));
    }

    #[test]
    fn test_winner_square_game_3() {
        let n = 4;
        assert_eq!(true, Solution::winner_square_game(n));
    }
}
