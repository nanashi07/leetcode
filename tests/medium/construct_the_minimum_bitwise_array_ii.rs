// 3315. Construct the Minimum Bitwise Array II
// https://leetcode.com/problems/construct-the-minimum-bitwise-array-ii/

struct Solution;

impl Solution {
    pub fn min_bitwise_array(nums: Vec<i32>) -> Vec<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::construct_the_minimum_bitwise_array_ii::Solution;

    #[test]
    fn test_min_bitwise_array_1() {
        let nums = [2, 3, 5, 7].to_vec();
        let output = [-1, 1, 4, 3].to_vec();
        assert_eq!(output, Solution::min_bitwise_array(nums));
    }

    #[test]
    fn test_min_bitwise_array_2() {
        let nums = [11, 13, 31].to_vec();
        let output = [9, 12, 15].to_vec();
        assert_eq!(output, Solution::min_bitwise_array(nums));
    }
}
