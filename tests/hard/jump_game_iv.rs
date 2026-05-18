// 1345. Jump Game IV
// https://leetcode.com/problems/jump-game-iv/

struct Solution;

impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::jump_game_iv::Solution;

    #[test]
    fn test_min_jumps_1() {
        let arr = [100, -23, -23, 404, 100, 23, 23, 23, 3, 404].to_vec();
        assert_eq!(3, Solution::min_jumps(arr));
    }

    #[test]
    fn test_min_jumps_2() {
        let arr = [7].to_vec();
        assert_eq!(0, Solution::min_jumps(arr));
    }

    #[test]
    fn test_min_jumps_3() {
        let arr = [7, 6, 9, 6, 9, 6, 9, 7].to_vec();
        assert_eq!(1, Solution::min_jumps(arr));
    }
}
