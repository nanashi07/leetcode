// 3689. Maximum Total Subarray Value I
// https://leetcode.com/problems/maximum-total-subarray-value-i/

struct Solution;

impl Solution {
    pub fn max_total_value(nums: Vec<i32>, k: i32) -> i64 {
        let mut min_num = nums[0];
        let mut max_num = nums[0];

        for &num in &nums[1..] {
            if num < min_num {
                min_num = num;
            }
            if num > max_num {
                max_num = num;
            }
        }

        (max_num as i64 - min_num as i64) * k as i64
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::maximum_total_subarray_value_i::Solution;

    #[test]
    fn test_max_total_value_1() {
        let nums = [1, 3, 2].to_vec();
        let k = 2;
        assert_eq!(4, Solution::max_total_value(nums, k));
    }

    #[test]
    fn test_max_total_value_2() {
        let nums = [4, 2, 5, 1].to_vec();
        let k = 3;
        assert_eq!(12, Solution::max_total_value(nums, k));
    }
}
