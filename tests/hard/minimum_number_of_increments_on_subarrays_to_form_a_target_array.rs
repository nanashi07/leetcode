// # 1526. Minimum Number of Increments on Subarrays to Form a Target Array
// https://leetcode.com/problems/minimum-number-of-increments-on-subarrays-to-form-a-target-array/

struct Solution;

impl Solution {
    pub fn min_number_operations(target: Vec<i32>) -> i32 {
        if target.is_empty() {
            return 0;
        }

        // Start with the first element - we need that many operations to build it
        let mut operations = target[0];

        // For each subsequent element, if it's higher than previous,
        // we need additional operations equal to the difference
        for i in 1..target.len() {
            if target[i] > target[i - 1] {
                operations += target[i] - target[i - 1];
            }
            // If target[i] <= target[i-1], we can reuse previous operations
        }

        operations
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::minimum_number_of_increments_on_subarrays_to_form_a_target_array::Solution;

    #[test]
    fn test_min_number_operations_1() {
        let target = [1, 2, 3, 2, 1].to_vec();
        assert_eq!(3, Solution::min_number_operations(target));
    }

    #[test]
    fn test_min_number_operations_2() {
        let target = [3, 1, 1, 2].to_vec();
        assert_eq!(4, Solution::min_number_operations(target));
    }

    #[test]
    fn test_min_number_operations_3() {
        let target = [3, 1, 5, 4, 2].to_vec();
        assert_eq!(7, Solution::min_number_operations(target));
    }
}
