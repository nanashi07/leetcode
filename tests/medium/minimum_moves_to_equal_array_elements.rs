// 453. Minimum Moves to Equal Array Elements
// https://leetcode.com/problems/minimum-moves-to-equal-array-elements/

struct Solution;

impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::minimum_moves_to_equal_array_elements::Solution;

    #[test]
    fn test_min_moves_1() {
        let nums = [1, 2, 3].to_vec();
        assert_eq!(3, Solution::min_moves(nums.to_vec()));
    }

    #[test]
    fn test_min_moves_2() {
        let nums = [1, 1, 1].to_vec();
        assert_eq!(0, Solution::min_moves(nums.to_vec()));
    }
}
