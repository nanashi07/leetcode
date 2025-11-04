// 3318. Find X-Sum of All K-Long Subarrays I
// https://leetcode.com/problems/find-x-sum-of-all-k-long-subarrays-i/

struct Solution;

impl Solution {
    pub fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::find_x_sum_of_all_k_long_subarrays_i::Solution;

    #[test]
    fn test_find_x_sum_1() {
        let nums = [1, 1, 2, 2, 3, 4, 2, 3].to_vec();
        let k = 6;
        let x = 2;
        let result = [6, 10, 12].to_vec();
        assert_eq!(result, Solution::find_x_sum(nums, k, x));
    }

    #[test]
    fn test_find_x_sum_2() {
        let nums = [3, 8, 7, 8, 7, 5].to_vec();
        let k = 2;
        let x = 2;
        let result = [11, 15, 15, 15, 12].to_vec();
        assert_eq!(result, Solution::find_x_sum(nums, k, x));
    }
}
