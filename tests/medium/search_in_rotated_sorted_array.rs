// 33. Search in Rotated Sorted Array
// https://leetcode.com/problems/search-in-rotated-sorted-array/
struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0usize;
        let mut right = nums.len();

        while left < right {
            let mid = left + (right - left) / 2;
            let value = nums[mid];

            if value == target {
                return mid as i32;
            }

            if nums[left] <= value {
                if nums[left] <= target && target < value {
                    right = mid;
                } else {
                    left = mid + 1;
                }
            } else if value < target && target <= nums[right - 1] {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        -1
    }
}

#[test]
fn test_search() {
    let nums = vec![4, 5, 6, 7, 0, 1, 2];
    let target = 0;
    let result = Solution::search(nums, target);
    assert_eq!(4, result);

    let nums = vec![4, 5, 6, 7, 0, 1, 2];
    let target = 3;
    let result = Solution::search(nums, target);
    assert_eq!(-1, result);

    let nums = vec![1];
    let target = 0;
    let result = Solution::search(nums, target);
    assert_eq!(-1, result);
}
