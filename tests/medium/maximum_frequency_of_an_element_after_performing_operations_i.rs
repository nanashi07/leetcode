// # 3346. Maximum Frequency of an Element After Performing Operations I
// https://leetcode.com/problems/maximum-frequency-of-an-element-after-performing-operations-i/

struct Solution;

impl Solution {
    pub fn max_frequency(nums: Vec<i32>, k: i32, num_operations: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::maximum_frequency_of_an_element_after_performing_operations_i::Solution;

    #[test]
    fn test_max_frequency_1() {
        let nums = [1, 4, 5].to_vec();
        let k = 1;
        let num_operations = 2;
        assert_eq!(2, Solution::max_frequency(nums, k, num_operations));
    }

    #[test]
    fn test_max_frequency_2() {
        let nums = [5, 11, 20, 20].to_vec();
        let k = 5;
        let num_operations = 1;
        assert_eq!(2, Solution::max_frequency(nums, k, num_operations));
    }
}
