// # 1493. Longest Subarray of 1's After Deleting One Element
// https://leetcode.com/problems/longest-subarray-of-1s-after-deleting-one-element/

struct Solution;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::longest_subarray_of_1s_after_deleting_one_element::Solution;

    #[test]
    fn test_longest_subarray_1() {
        let nums = [1, 1, 0, 1].to_vec();
        assert_eq!(3, Solution::longest_subarray(nums));
    }

    #[test]
    fn test_longest_subarray_2() {
        let nums = [0, 1, 1, 1, 0, 1, 1, 0, 1].to_vec();
        assert_eq!(5, Solution::longest_subarray(nums));
    }

    #[test]
    fn test_longest_subarray_3() {
        let nums = [1, 1, 1].to_vec();
        assert_eq!(2, Solution::longest_subarray(nums));
    }
}
