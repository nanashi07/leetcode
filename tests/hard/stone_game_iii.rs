// 1406. Stone Game III
// https://leetcode.com/problems/stone-game-iii/

struct Solution;

impl Solution {
    pub fn stone_game_iii(stone_value: Vec<i32>) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::stone_game_iii::Solution;

    #[test]
    fn test_stone_game_iii_1() {
        let stone_value = [1, 2, 3, 7].to_vec();
        assert_eq!("Bob".to_string(), Solution::stone_game_iii(stone_value));
    }

    #[test]
    fn test_stone_game_iii_2() {
        let stone_value = [1, 2, 3, -9].to_vec();
        assert_eq!("Alice".to_string(), Solution::stone_game_iii(stone_value));
    }

    #[test]
    fn test_stone_game_iii_3() {
        let stone_value = [1, 2, 3, 6].to_vec();
        assert_eq!("Tie".to_string(), Solution::stone_game_iii(stone_value));
    }
}
