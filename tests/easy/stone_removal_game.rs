// 3360. Stone Removal Game
// https://leetcode.com/problems/stone-removal-game/

struct Solution;

impl Solution {
    pub fn can_alice_win(n: i32) -> bool {
        let mut remaining = n;
        let mut remove = 10;
        let mut moves = 0;

        while remove > 0 && remaining >= remove {
            remaining -= remove;
            remove -= 1;
            moves += 1;
        }

        // The player unable to move loses, so Alice wins iff she made the last legal move.
        moves % 2 == 1
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
