// # 33. Search in Rotated Sorted Array
// https://leetcode.com/problems/search-in-rotated-sorted-array/
struct Solution;

impl Solution {
    // https://leetcode.com/problems/search-in-rotated-sorted-array/discuss/14425/Concise-O(log-N)-Binary-search-solution
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.len() == 0 {
            return -1;
        }
        let mut l: i32 = 0;
        let mut r: i32 = nums.len() as i32 - 1;
        let mut m: i32;
        // find out the index of the smallest element.
        while l < r {
            m = (l + r) / 2;
            if nums[m as usize] > nums[r as usize] {
                l = m + 1;
            } else {
                r = m;
            }
        }

        // since we now know the start, find out if the target is to left or right of start in the array.
        let s = l;
        l = 0;
        r = nums.len() as i32 - 1;
        if target >= nums[s as usize] && target <= nums[r as usize] {
            l = s;
        } else {
            r = s;
        }
        // the regular search.
        while l <= r {
            m = (l + r) / 2;
            if nums[m as usize] == target {
                return m as i32;
            } else if nums[m as usize] > target {
                r = m - 1;
            } else {
                l = m + 1;
            }
        }

        return -1;
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
