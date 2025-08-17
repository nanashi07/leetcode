// # 837. New 21 Game
// https://leetcode.com/problems/new-21-game/description/?envType=daily-question&envId=2025-08-17

struct Solution;

impl Solution {
    pub fn new21_game(n: i32, k: i32, max_pts: i32) -> f64 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::new_21_game::Solution;

    #[test]
    fn test_new21_game_1() {
        let n = 10;
        let k = 1;
        let max_pts = 10;
        assert_eq!(1.00000, Solution::new21_game(n, k, max_pts));
    }

    #[test]
    fn test_new21_game_2() {
        let n = 6;
        let k = 1;
        let max_pts = 10;
        assert_eq!(0.60000, Solution::new21_game(n, k, max_pts));
    }

    #[test]
    fn test_new21_game_3() {
        let n = 21;
        let k = 17;
        let max_pts = 10;
        assert_eq!(0.73278, Solution::new21_game(n, k, max_pts));
    }
}
