// 3736. Minimum Moves to Equal Array Elements III
// https://leetcode.com/problems/minimum-moves-to-equal-array-elements-iii/

struct Solution;

impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        let max = nums.iter().max().unwrap();
        nums.iter().map(|&x| max - x).sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::minimum_moves_to_equal_array_elements_iii::Solution;

    #[test]
    fn test_min_moves_1() {
        let nums = [2, 1, 3].to_vec();
        assert_eq!(3, Solution::min_moves(nums));
    }

    #[test]
    fn test_min_moves_2() {
        let nums = [4, 4, 5].to_vec();
        assert_eq!(2, Solution::min_moves(nums));
    }
}
