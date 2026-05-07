// 3660. Jump Game IX
// https://leetcode.com/problems/jump-game-ix/

struct Solution;

impl Solution {
    pub fn max_value(nums: Vec<i32>) -> Vec<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::jump_game_ix::Solution;

    #[test]
    fn test_max_value_1() {
        let nums = [2, 1, 3].to_vec();
        assert_eq!([2, 2, 3].to_vec(), Solution::max_value(nums));
    }

    #[test]
    fn test_max_value_2() {
        let nums = [2, 3, 1].to_vec();
        assert_eq!([3, 3, 3].to_vec(), Solution::max_value(nums));
    }
}
