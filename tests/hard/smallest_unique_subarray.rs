// 3934. Smallest Unique Subarray
// https://leetcode.com/problems/smallest-unique-subarray/

struct Solution;

impl Solution {
    pub fn smallest_unique_subarray(nums: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::smallest_unique_subarray::Solution;

    #[test]
    fn test_smallest_unique_subarray_1() {
        let nums: Vec<i32> = vec![3, 3, 3];
        assert_eq!(3, Solution::smallest_unique_subarray(nums));
    }

    #[test]
    fn test_smallest_unique_subarray_2() {
        let nums: Vec<i32> = vec![2, 1, 2, 3, 3];
        assert_eq!(1, Solution::smallest_unique_subarray(nums));
    }

    #[test]
    fn test_smallest_unique_subarray_3() {
        let nums: Vec<i32> = vec![1, 1, 2, 2, 1];
        assert_eq!(2, Solution::smallest_unique_subarray(nums));
    }
}
