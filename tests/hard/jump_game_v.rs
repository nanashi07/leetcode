// 1340. Jump Game V
// https://leetcode.com/problems/jump-game-v/

struct Solution;

impl Solution {
    pub fn max_jumps(arr: Vec<i32>, d: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::jump_game_v::Solution;

    #[test]
    fn test_max_jumps_1() {
        let arr = [6, 4, 14, 6, 8, 13, 9, 7, 10, 6, 12].to_vec();
        let d = 2;
        assert_eq!(4, Solution::max_jumps(arr, d));
    }

    #[test]
    fn test_max_jumps_2() {
        let arr = [3, 3, 3, 3, 3].to_vec();
        let d = 3;
        assert_eq!(1, Solution::max_jumps(arr, d));
    }

    #[test]
    fn test_max_jumps_3() {
        let arr = [7, 6, 5, 4, 3, 2, 1].to_vec();
        let d = 1;
        assert_eq!(7, Solution::max_jumps(arr, d));
    }
}
