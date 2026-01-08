// 1458. Max Dot Product of Two Subsequences
// https://leetcode.com/problems/max-dot-product-of-two-subsequences/

struct Solution;

impl Solution {
    pub fn max_dot_product(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::max_dot_product_of_two_subsequences::Solution;

    #[test]
    fn test_max_dot_product_1() {
        let nums1 = [2, 1, -2, 5].to_vec();
        let nums2 = [3, 0, -6].to_vec();
        assert_eq!(18, Solution::max_dot_product(nums1, nums2));
    }

    #[test]
    fn test_max_dot_product_2() {
        let nums1 = [3, -2].to_vec();
        let nums2 = [2, -6, 7].to_vec();
        assert_eq!(21, Solution::max_dot_product(nums1, nums2));
    }

    #[test]
    fn test_max_dot_product_3() {
        let nums1 = [-1, -1].to_vec();
        let nums2 = [1, 1].to_vec();
        assert_eq!(-1, Solution::max_dot_product(nums1, nums2));
    }
}
