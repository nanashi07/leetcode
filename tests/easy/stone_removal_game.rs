// 3360. Stone Removal Game
// https://leetcode.com/problems/stone-removal-game/

struct Solution;

impl Solution {
    pub fn can_alice_win(n: i32) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::stone_removal_game::Solution;

    #[test]
    fn test_can_alice_win_1() {
        let n: i32 = 12;
        assert!(Solution::can_alice_win(n));
    }

    #[test]
    fn test_can_alice_win_2() {
        let n: i32 = 1;
        assert!(!Solution::can_alice_win(n));
    }
}
