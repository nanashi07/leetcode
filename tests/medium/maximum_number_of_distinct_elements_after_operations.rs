// # 3397. Maximum Number of Distinct Elements After Operations
// https://leetcode.com/problems/maximum-number-of-distinct-elements-after-operations/

struct Solution;

impl Solution {
    pub fn max_distinct_elements(nums: Vec<i32>, k: i32) -> i32 {
        todo!()
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
