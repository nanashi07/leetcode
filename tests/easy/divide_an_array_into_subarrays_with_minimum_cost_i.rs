// 3010. Divide an Array Into Subarrays With Minimum Cost I
// https://leetcode.com/problems/divide-an-array-into-subarrays-with-minimum-cost-i/

struct Solution;

impl Solution {
    pub fn minimum_cost(nums: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::divide_an_array_into_subarrays_with_minimum_cost_i::Solution;

    #[test]
    fn test_minimum_cost_1() {
        let nums = [1, 2, 3, 12].to_vec();
        assert_eq!(6, Solution::minimum_cost(nums));
    }

    #[test]
    fn test_minimum_cost_2() {
        let nums = [5, 4, 3].to_vec();
        assert_eq!(12, Solution::minimum_cost(nums));
    }

    #[test]
    fn test_minimum_cost_3() {
        let nums = [10, 3, 1, 1].to_vec();
        assert_eq!(12, Solution::minimum_cost(nums));
    }
}
