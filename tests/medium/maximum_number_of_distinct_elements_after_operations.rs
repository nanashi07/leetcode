// 3397. Maximum Number of Distinct Elements After Operations
// https://leetcode.com/problems/maximum-number-of-distinct-elements-after-operations/

struct Solution;

impl Solution {
    pub fn max_distinct_elements(nums: Vec<i32>, k: i32) -> i32 {
        println!("nums: {:?}, k: {}", &nums, &k);

        let mut nums = nums;
        nums.sort_unstable();

        let mut distinct_count = 0;
        let mut last_used = i32::MIN;

        for num in nums {
            // The range we can transform this number to is [num - k, num + k]
            let min_val = num - k;
            let max_val = num + k;

            // We want to use the smallest value >= last_used + 1 that's in range
            let target = (last_used + 1).max(min_val);

            // Check if target is within the allowed range
            if target <= max_val {
                last_used = target;
                distinct_count += 1;
            }
            // If target > max_val, we can't make this number distinct
        }

        distinct_count
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::maximum_number_of_distinct_elements_after_operations::Solution;

    #[test]
    fn test_max_distinct_elements_1() {
        let nums = [1, 2, 2, 3, 3, 4].to_vec();
        let k = 2;
        assert_eq!(6, Solution::max_distinct_elements(nums, k));
    }

    #[test]
    fn test_max_distinct_elements_2() {
        let nums = [4, 4, 4, 4].to_vec();
        let k = 1;
        assert_eq!(3, Solution::max_distinct_elements(nums, k));
    }
}
