// 2897. Apply Operations on Array to Maximize Sum of Squares
// https://leetcode.com/problems/apply-operations-on-array-to-maximize-sum-of-squares/

struct Solution;

impl Solution {
    pub fn max_sum(nums: Vec<i32>, k: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::apply_operations_on_array_to_maximize_sum_of_squares::Solution;

    #[test]
    fn test_max_sum_1() {
        let nums = [2, 6, 5, 8].to_vec();
        let k = 2;
        assert_eq!(261, Solution::max_sum(nums, k));
    }

    #[test]
    fn test_max_sum_2() {
        let nums = [4, 5, 4, 7].to_vec();
        let k = 3;
        assert_eq!(90, Solution::max_sum(nums, k));
    }
}
