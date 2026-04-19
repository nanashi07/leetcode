// 1855. Maximum Distance Between a Pair of Values
// https://leetcode.com/problems/maximum-distance-between-a-pair-of-values/

struct Solution;

impl Solution {
    pub fn max_distance(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut result = 0;
        let (mut i, mut j) = (0, 0);
        while i < nums1.len() && j < nums2.len() {
            if nums1[i] <= nums2[j] {
                result = result.max(j - i);
                j += 1;
            } else {
                i += 1;
                if j < i {
                    j = i;
                }
            }
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::maximum_distance_between_a_pair_of_values::Solution;

    #[test]
    fn test_max_distance_1() {
        let nums1 = [55, 30, 5, 4, 2].to_vec();
        let nums2 = [100, 20, 10, 10, 5].to_vec();
        assert_eq!(2, Solution::max_distance(nums1, nums2));
    }

    #[test]
    fn test_max_distance_2() {
        let nums1 = [2, 2, 2].to_vec();
        let nums2 = [10, 10, 1].to_vec();
        assert_eq!(1, Solution::max_distance(nums1, nums2));
    }

    #[test]
    fn test_max_distance_3() {
        let nums1 = [30, 29, 19, 5].to_vec();
        let nums2 = [25, 25, 25, 25, 25].to_vec();
        assert_eq!(2, Solution::max_distance(nums1, nums2));
    }
}
