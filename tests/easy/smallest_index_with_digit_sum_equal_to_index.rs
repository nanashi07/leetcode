// 3550. Smallest Index With Digit Sum Equal to Index
// https://leetcode.com/problems/smallest-index-with-digit-sum-equal-to-index/

struct Solution;

impl Solution {
    pub fn smallest_index(nums: Vec<i32>) -> i32 {
        let zero = '0' as usize;
        for (i, &n) in nums.iter().enumerate() {
            if i == n
                .to_string()
                .chars()
                .map(|c| c as usize - zero)
                .sum::<usize>()
            {
                return i as i32;
            }
        }
        -1
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
