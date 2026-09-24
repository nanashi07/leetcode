// 3550. Smallest Index With Digit Sum Equal to Index
// https://leetcode.com/problems/smallest-index-with-digit-sum-equal-to-index/

struct Solution;

impl Solution {
    pub fn smallest_index(nums: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::smallest_index_with_digit_sum_equal_to_index::Solution;

    #[test]
    fn test_smallest_index_1() {
        let nums = [1, 3, 2].to_vec();
        assert_eq!(2, Solution::smallest_index(nums));
    }

    #[test]
    fn test_smallest_index_2() {
        let nums = [1, 10, 11].to_vec();
        assert_eq!(1, Solution::smallest_index(nums));
    }

    #[test]
    fn test_smallest_index_3() {
        let nums = [1, 2, 3].to_vec();
        assert_eq!(-1, Solution::smallest_index(nums));
    }
}
