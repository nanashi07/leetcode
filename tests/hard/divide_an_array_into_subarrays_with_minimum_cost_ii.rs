// 3013. Divide an Array Into Subarrays With Minimum Cost II
// https://leetcode.com/problems/divide-an-array-into-subarrays-with-minimum-cost-ii/

struct Solution;

impl Solution {
    pub fn minimum_cost(nums: Vec<i32>, k: i32, dist: i32) -> i64 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::divide_an_array_into_subarrays_with_minimum_cost_ii::Solution;

    #[test]
    fn test_minimum_cost_1() {
        let nums = [1, 3, 2, 6, 4, 2].to_vec();
        let k = 3;
        let dist = 3;
        assert_eq!(5, Solution::minimum_cost(nums, k, dist));
    }

    #[test]
    fn test_minimum_cost_2() {
        let nums = [10, 1, 2, 2, 2, 1].to_vec();
        let k = 4;
        let dist = 3;
        assert_eq!(15, Solution::minimum_cost(nums, k, dist));
    }

    #[test]
    fn test_minimum_cost_3() {
        let nums = [10, 8, 18, 9].to_vec();
        let k = 3;
        let dist = 1;
        assert_eq!(36, Solution::minimum_cost(nums, k, dist));
    }
}
