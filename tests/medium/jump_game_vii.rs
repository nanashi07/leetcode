// 1871. Jump Game VII
// https://leetcode.com/problems/jump-game-vii/

struct Solution;

impl Solution {
    pub fn can_reach(s: String, min_jump: i32, max_jump: i32) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::jump_game_vii::Solution;

    #[test]
    fn test_can_reach_1() {
        let s = "011010".to_string();
        let min_jump = 2;
        let max_jump = 3;
        assert_eq!(true, Solution::can_reach(s, min_jump, max_jump));
    }

    #[test]
    fn test_can_reach_2() {
        let s = "01101110".to_string();
        let min_jump = 2;
        let max_jump = 3;
        assert_eq!(false, Solution::can_reach(s, min_jump, max_jump));
    }
}
