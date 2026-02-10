// 3719. Longest Balanced Subarray I
// https://leetcode.com/problems/longest-balanced-subarray-i/

struct Solution;

impl Solution {
    pub fn longest_balanced(nums: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::longest_balanced_subarray_i::Solution;

    #[test]
    fn test_longest_balanced_1() {
        let nums = [2, 5, 4, 3].to_vec();
        assert_eq!(4, Solution::longest_balanced(nums));
    }

    #[test]
    fn test_longest_balanced_2() {
        let nums = [3, 2, 2, 5, 4].to_vec();
        assert_eq!(5, Solution::longest_balanced(nums));
    }

    #[test]
    fn test_longest_balanced_3() {
        let nums = [1, 2, 3, 2].to_vec();
        assert_eq!(3, Solution::longest_balanced(nums));
    }
}
