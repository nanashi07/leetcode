// 2540. Minimum Common Value
// https://leetcode.com/problems/minimum-common-value/

struct Solution;

impl Solution {
    pub fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut i1 = 0;
        let mut i2 = 0;
        while i1 < nums1.len() && i2 < nums2.len() {
            let n1 = nums1[i1];
            let n2 = nums2[i2];
            if n1 == n2 {
                return n1;
            } else if n1 < n2 {
                i1 += 1;
            } else {
                i2 += 1;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::minimum_common_value::Solution;

    #[test]
    fn test_get_common_1() {
        let nums1 = [1, 2, 3].to_vec();
        let nums2 = [2, 4].to_vec();
        assert_eq!(2, Solution::get_common(nums1, nums2));
    }

    #[test]
    fn test_get_common_2() {
        let nums1 = [1, 2, 3, 6].to_vec();
        let nums2 = [2, 3, 4, 5].to_vec();
        assert_eq!(2, Solution::get_common(nums1, nums2));
    }
}
