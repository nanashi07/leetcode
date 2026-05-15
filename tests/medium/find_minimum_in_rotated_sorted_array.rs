// 153. Find Minimum in Rotated Sorted Array
// https://leetcode.com/problems/find-minimum-in-rotated-sorted-array/

struct Solution;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut left = 0usize;
        let mut right = nums.len() - 1;

        while left < right {
            let mid = left + (right - left) / 2;
            if nums[mid] > nums[right] {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        nums[left]
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::find_minimum_in_rotated_sorted_array::Solution;

    #[test]
    fn test_find_min_1() {
        let nums = [3, 4, 5, 1, 2].to_vec();
        assert_eq!(1, Solution::find_min(nums));
    }

    #[test]
    fn test_find_min_2() {
        let nums = [4, 5, 6, 7, 0, 1, 2].to_vec();
        assert_eq!(0, Solution::find_min(nums));
    }

    #[test]
    fn test_find_min_3() {
        let nums = [11, 13, 15, 17].to_vec();
        assert_eq!(11, Solution::find_min(nums));
    }
}
