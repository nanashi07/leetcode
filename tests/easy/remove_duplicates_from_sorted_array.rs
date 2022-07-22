// # 26. Remove Duplicates from Sorted Array
// https://leetcode.com/problems/remove-duplicates-from-sorted-array/
struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut i = 0;
        // let mut moved = 0;
        let mut len = nums.len();
        while i + 1 < len {
            if nums[i] == nums[i + 1] && i + 1 < len {
                for j in i + 1..len - 1 {
                    nums[j] = nums[j + 1];
                }
                len = len - 1;
            } else {
                i = i + 1;
            }
        }

        len as i32
    }
    // https://leetcode.com/problems/remove-duplicates-from-sorted-array/discuss/11782/Share-my-clean-C%2B%2B-code
    fn remove_duplicates_from_discuss(nums: &mut Vec<i32>) -> i32 {
        let mut count = 0;
        println!("c: {}, v: {:?}", count, &nums);
        for i in 1..nums.len() {
            if nums[i] == nums[i - 1] {
                // sum duplicated count
                count = count + 1;
            } else {
                // copy value from next difference to `next position`
                println!("{:?}", &nums);
                nums[i - count] = nums[i];
                println!("{:?}", &nums);
            };
            println!("i: {}, c: {}, v: {:?}", i, count, &nums);
        }
        println!("------------------------");
        (nums.len() - count) as i32
    }
}

#[test]
fn test_remove_duplicates() {
    let mut nums = vec![1, 1, 2];
    let result = Solution::remove_duplicates(&mut nums);
    assert_eq!(2, result);

    let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    let result = Solution::remove_duplicates(&mut nums);
    assert_eq!(5, result);

    let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 2, 3, 3, 4];
    let result = Solution::remove_duplicates(&mut nums);
    assert_eq!(5, result);

    let mut nums = vec![1, 1];
    let result = Solution::remove_duplicates(&mut nums);
    assert_eq!(1, result);
}

#[test]
fn test_remove_duplicates_from_discuss() {
    let mut nums = vec![1, 1, 2];
    let result = Solution::remove_duplicates_from_discuss(&mut nums);
    assert_eq!(2, result);

    let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    let result = Solution::remove_duplicates_from_discuss(&mut nums);
    assert_eq!(5, result);

    let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 2, 3, 3, 4];
    let result = Solution::remove_duplicates_from_discuss(&mut nums);
    assert_eq!(5, result);

    let mut nums = vec![1, 1];
    let result = Solution::remove_duplicates_from_discuss(&mut nums);
    assert_eq!(1, result);
}
