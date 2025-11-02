// 34. Find First and Last Position of Element in Sorted Array
// https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/
struct Solution;

impl Solution {
    // https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/discuss/14699/Clean-iterative-solution-with-two-binary-searches-(with-explanation)
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.is_empty() {
            return vec![-1, -1];
        }

        let mut la: i32 = -1;
        let mut ra: i32 = -1;

        let mut l = 0;
        let mut r = nums.len() - 1;
        let mut m;

        // Search for the left one
        while l < r {
            m = (l + r) / 2;
            if nums[m] < target {
                l = m + 1;
            } else {
                r = m;
            }
        }
        if nums[l] != target {
            return vec![la, ra];
        } else {
            la = l as i32;
        }

        let mut r = nums.len() - 1; // We don't have to set i to 0 the second time.

        // Search for the right one
        while l < r {
            m = (l + r) / 2 + 1; // Make mid biased to the right
            if nums[m] > target {
                r = m - 1;
            } else {
                l = m; // So that this won't make the search range stuck.
            }
        }

        ra = r as i32;

        vec![la, ra]
    }
}

#[test]
fn test_search_range() {
    let nums = vec![5, 7, 7, 8, 8, 10];
    let target = 8;
    let result = Solution::search_range(nums, target);
    assert_eq!(vec![3, 4], result);

    let nums = vec![5, 7, 7, 8, 8, 10];
    let target = 6;
    let result = Solution::search_range(nums, target);
    assert_eq!(vec![-1, -1], result);

    let nums = vec![];
    let target = 0;
    let result = Solution::search_range(nums, target);
    assert_eq!(vec![-1, -1], result);

    let nums = vec![1];
    let target = 0;
    let result = Solution::search_range(nums, target);
    assert_eq!(vec![-1, -1], result);

    let nums = vec![2, 2];
    let target = 2;
    let result = Solution::search_range(nums, target);
    assert_eq!(vec![0, 1], result);

    let nums = vec![1, 1, 2];
    let target = 1;
    let result = Solution::search_range(nums, target);
    assert_eq!(vec![0, 1], result);

    let nums = vec![1, 2, 3, 3, 3, 3, 4, 5, 9];
    let target = 3;
    let result = Solution::search_range(nums, target);
    assert_eq!(vec![2, 5], result);
}
