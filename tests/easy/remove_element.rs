// # 27. Remove Element
// https://leetcode.com/problems/remove-element/
struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut count = 0;

        for i in 0..nums.len() {
            if nums[i] != val {
                nums[count] = nums[i];
                count = count + 1;
            }
        }

        count as i32
    }
}

#[test]
fn test_remove_element() {
    let mut nums = vec![3, 2, 2, 3];
    let val = 3;
    let result = Solution::remove_element(&mut nums, val);
    assert_eq!(2, result);

    let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
    let val = 2;
    let result = Solution::remove_element(&mut nums, val);
    assert_eq!(5, result);
}
