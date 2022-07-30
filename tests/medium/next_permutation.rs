// # 31. Next Permutation
// https://leetcode.com/problems/next-permutation/
struct Solution;

impl Solution {
    // https://leetcode.com/problems/next-permutation/discuss/13867/C%2B%2B-from-Wikipedia
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let n = nums.len();
        let mut k = n;
        for i in (0..n - 1).rev() {
            if nums[i] < nums[i + 1] {
                k = i;
                break;
            }
        }
        if k == n {
            Solution::reverse_suffix(nums, 0);
        } else {
            for i in (k + 1..n).rev() {
                if nums[i] > nums[k] {
                    Solution::swap(nums, k, i);
                    break;
                }
            }
            Solution::reverse_suffix(nums, k + 1);
        }
    }

    fn swap(nums: &mut Vec<i32>, i: usize, j: usize) {
        let tmp = nums[i];
        nums[i] = nums[j];
        nums[j] = tmp;
    }

    fn reverse_suffix(nums: &mut Vec<i32>, start: usize) {
        let mut start = start;
        let mut end = nums.len() - 1;
        while start < end {
            Solution::swap(nums, start, end);
            start = start + 1;
            end = end - 1;
        }
    }
}

#[test]
fn test_next_permutation() {
    let mut nums = vec![1, 2, 3];
    Solution::next_permutation(&mut nums);
    let expected = vec![1, 3, 2];
    assert_eq!(expected, nums);

    let mut nums = vec![3, 2, 1];
    Solution::next_permutation(&mut nums);
    let expected = vec![1, 2, 3];
    assert_eq!(expected, nums);

    let mut nums = vec![1, 1, 5];
    Solution::next_permutation(&mut nums);
    let expected = vec![1, 5, 1];
    assert_eq!(expected, nums);

    let mut nums = vec![1, 3, 2];
    Solution::next_permutation(&mut nums);
    let expected = vec![2, 1, 3];
    assert_eq!(expected, nums);

    let mut nums = vec![0, 1, 2, 5, 3, 3, 0];
    Solution::next_permutation(&mut nums);
    let expected = vec![0, 1, 3, 0, 2, 3, 5];
    assert_eq!(expected, nums);

    let mut nums = vec![2, 1, 3];
    Solution::next_permutation(&mut nums);
    let expected = vec![2, 3, 1];
    assert_eq!(expected, nums);
}
