// # 2419. Longest Subarray With Maximum Bitwise AND
// https://leetcode.com/problems/longest-subarray-with-maximum-bitwise-and/

struct Solution;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::longest_subarray_with_maximum_bitwise_and::Solution;

    #[test]
    fn test_longest_subarray_1() {
        let nums = [1, 2, 3, 3, 2, 2].to_vec();
        assert_eq!(2, Solution::longest_subarray(nums));
    }

    #[test]
    fn test_longest_subarray_2() {
        let nums = [1, 2, 3, 4].to_vec();
        assert_eq!(1, Solution::longest_subarray(nums));
    }
}
