// 1563. Stone Game V
// https://leetcode.com/problems/stone-game-v/

struct Solution;

impl Solution {
    pub fn stone_game_v(stone_value: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::stone_game_v::Solution;

    #[test]
    fn test_stone_game_v_1() {
        let stone_value = [6, 2, 3, 4, 5, 5].to_vec();
        assert_eq!(28, Solution::stone_game_v(stone_value));
    }

    #[test]
    fn test_stone_game_v_2() {
        let stone_value = [4].to_vec();
        assert_eq!(0, Solution::stone_game_v(stone_value));
    }
}
