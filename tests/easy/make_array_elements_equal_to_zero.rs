// # 3354. Make Array Elements Equal to Zero
// https://leetcode.com/problems/make-array-elements-equal-to-zero/

struct Solution;

impl Solution {
    pub fn count_valid_selections(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let total_sum: i32 = nums.iter().sum();
        let mut count = 0;
        let mut left_sum = 0;

        for i in 0..n {
            if nums[i] == 0 {
                let right_sum = total_sum - left_sum;

                // If left_sum == right_sum, both directions work
                if left_sum == right_sum {
                    count += 2;
                }
                // If difference is 1, one direction works
                else if (left_sum - right_sum).abs() == 1 {
                    count += 1;
                }
                // If difference > 1, neither direction works
            }
            left_sum += nums[i];
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::make_array_elements_equal_to_zero::Solution;

    #[test]
    fn test_count_valid_selections_1() {
        let nums = [1, 0, 2, 0, 3].to_vec();
        assert_eq!(2, Solution::count_valid_selections(nums));
    }

    #[test]
    fn test_count_valid_selections_2() {
        let nums = [2, 3, 4, 0, 4, 1, 0].to_vec();
        assert_eq!(0, Solution::count_valid_selections(nums));
    }
}
