// # 3021. Alice and Bob Playing Flower Game
// https://leetcode.com/problems/alice-and-bob-playing-flower-game/

struct Solution;

impl Solution {
    pub fn flower_game(n: i32, m: i32) -> i64 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::alice_and_bob_playing_flower_game::Solution;

    #[test]
    fn test_flower_game_1() {
        let n = 3;
        let m = 2;
        assert_eq!(3, Solution::flower_game(n, m));
    }

    #[test]
    fn test_flower_game_2() {
        let n = 1;
        let m = 1;
        assert_eq!(0, Solution::flower_game(n, m));
    }
}
