// # 611. Valid Triangle Number
// https://leetcode.com/problems/valid-triangle-number/

struct Solution;

impl Solution {
    pub fn triangle_number(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();
        let n = nums.len();
        let mut count = 0;

        // For each potential largest side (from right to left)
        for k in (2..n).rev() {
            let mut left = 0;
            let mut right = k - 1;

            // Two pointer approach to find valid pairs
            while left < right {
                if nums[left] + nums[right] > nums[k] {
                    // If nums[left] + nums[right] > nums[k], then
                    // nums[left+1] + nums[right] > nums[k], nums[left+2] + nums[right] > nums[k], etc.
                    // So all pairs (left, right), (left+1, right), ..., (right-1, right) are valid
                    count += right - left;
                    right -= 1;
                } else {
                    // nums[left] + nums[right] <= nums[k], need larger sum
                    left += 1;
                }
            }
        }

        count as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::valid_triangle_number::Solution;

    #[test]
    fn test_triangle_number_1() {
        let nums = [2, 2, 3, 4].to_vec();
        assert_eq!(3, Solution::triangle_number(nums));
    }

    #[test]
    fn test_triangle_number_2() {
        let nums = [4, 2, 3, 4].to_vec();
        assert_eq!(4, Solution::triangle_number(nums));
    }
}
