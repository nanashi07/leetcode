// 3411. Maximum Subarray With Equal Products
// https://leetcode.com/problems/maximum-subarray-with-equal-products/

struct Solution;

impl Solution {
    pub fn max_length(nums: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::maximum_subarray_with_equal_products::Solution;

    #[test]
    fn test_max_length_1() {
        let nums: Vec<i32> = vec![1, 2, 1, 2, 1, 1, 1];
        assert_eq!(5, Solution::max_length(nums));
    }

    #[test]
    fn test_max_length_2() {
        let nums: Vec<i32> = vec![2, 3, 4, 5, 6];
        assert_eq!(3, Solution::max_length(nums));
    }

    #[test]
    fn test_max_length_3() {
        let nums: Vec<i32> = vec![1, 2, 3, 1, 4, 5, 1];
        assert_eq!(5, Solution::max_length(nums));
    }
}
