// 1306. Jump Game III
// https://leetcode.com/problems/jump-game-iii/

struct Solution;

impl Solution {
    pub fn can_reach(arr: Vec<i32>, start: i32) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::jump_game_iii::Solution;

    #[test]
    fn test_can_reach_1() {
        let arr = [4, 2, 3, 0, 3, 1, 2].to_vec();
        let start = 5;
        assert_eq!(true, Solution::can_reach(arr, start));
    }

    #[test]
    fn test_can_reach_2() {
        let arr = [4, 2, 3, 0, 3, 1, 2].to_vec();
        let start = 0;
        assert_eq!(true, Solution::can_reach(arr, start));
    }

    #[test]
    fn test_can_reach_3() {
        let arr = [3, 0, 2, 1, 2].to_vec();
        let start = 2;
        assert_eq!(false, Solution::can_reach(arr, start));
    }
}
