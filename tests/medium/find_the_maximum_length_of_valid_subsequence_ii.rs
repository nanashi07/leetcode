// # 3202. Find the Maximum Length of Valid Subsequence II
// https://leetcode.com/problems/find-the-maximum-length-of-valid-subsequence-ii/

struct Solution;

impl Solution {
    pub fn maximum_length(nums: Vec<i32>, k: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::find_the_maximum_length_of_valid_subsequence_ii::Solution;

    #[test]
    fn test_maximum_length_1() {
        let nums = [1, 2, 3, 4, 5].to_vec();
        let k = 2;
        assert_eq!(5, Solution::maximum_length(nums, k));
    }

    #[test]
    fn test_maximum_length_2() {
        let nums = [1, 4, 2, 3, 1, 4].to_vec();
        let k = 3;
        assert_eq!(4, Solution::maximum_length(nums, k));
    }
}
