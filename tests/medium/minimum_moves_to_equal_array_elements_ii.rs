// 462. Minimum Moves to Equal Array Elements II
// https://leetcode.com/problems/minimum-moves-to-equal-array-elements-ii/

struct Solution;

impl Solution {
    pub fn min_moves2(nums: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::minimum_moves_to_equal_array_elements_ii::Solution;

    #[test]
    fn test_min_moves2_1() {
        let nums = [1, 2, 3].to_vec();
        assert_eq!(2, Solution::min_moves2(nums));
    }

    #[test]
    fn test_min_moves2_2() {
        let nums = [1, 10, 2, 9].to_vec();
        assert_eq!(16, Solution::min_moves2(nums));
    }
}
