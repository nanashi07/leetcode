// 1658. Minimum Operations to Reduce X to Zero
// https://leetcode.com/problems/minimum-operations-to-reduce-x-to-zero/

struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        // Taking elements only from the two ends is equivalent to leaving one
        // contiguous middle subarray whose sum equals `total - x`. Minimizing
        // the removed count means maximizing that middle window.
        let target = nums.iter().map(|&v| v as i64).sum::<i64>() - x as i64;
        if target < 0 {
            return -1;
        }

        // -1 marks "no valid middle window found", which is distinct from the
        // legitimate zero-length window returned when `target == 0`.
        let mut longest = -1isize;
        let mut window_sum = 0i64;
        let mut left = 0usize;

        for right in 0..nums.len() {
            window_sum += nums[right] as i64;
            // All values are positive, so shrink from the left while over budget.
            while window_sum > target {
                window_sum -= nums[left] as i64;
                left += 1;
            }
            if window_sum == target {
                longest = longest.max((right + 1 - left) as isize);
            }
        }

        if longest < 0 {
            -1
        } else {
            nums.len() as i32 - longest as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::minimum_operations_to_reduce_x_to_zero::Solution;

    #[test]
    fn test_min_operations_1() {
        let nums = [1, 1, 4, 2, 3].to_vec();
        let x = 5;
        assert_eq!(2, Solution::min_operations(nums, x));
    }

    #[test]
    fn test_min_operations_2() {
        let nums = [5, 6, 7, 8, 9].to_vec();
        let x = 4;
        assert_eq!(-1, Solution::min_operations(nums, x));
    }

    #[test]
    fn test_min_operations_3() {
        let nums = [3, 2, 20, 1, 1, 3].to_vec();
        let x = 10;
        assert_eq!(5, Solution::min_operations(nums, x));
    }
}
