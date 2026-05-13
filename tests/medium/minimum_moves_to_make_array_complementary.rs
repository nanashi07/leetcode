// 1674. Minimum Moves to Make Array Complementary
// https://leetcode.com/problems/minimum-moves-to-make-array-complementary/

struct Solution;

impl Solution {
    pub fn min_moves(nums: Vec<i32>, limit: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::minimum_moves_to_make_array_complementary::Solution;

    #[test]
    fn test_min_moves_1() {
        let nums = [1, 2, 4, 3].to_vec();
        let limit = 4;
        assert_eq!(1, Solution::min_moves(nums, limit));
    }

    #[test]
    fn test_min_moves_2() {
        let nums = [1, 2, 2, 1].to_vec();
        let limit = 2;
        assert_eq!(2, Solution::min_moves(nums, limit));
    }

    #[test]
    fn test_min_moves_3() {
        let nums = [1, 2, 1, 2].to_vec();
        let limit = 2;
        assert_eq!(0, Solution::min_moves(nums, limit));
    }
}
