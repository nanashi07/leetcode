// 1590. Make Sum Divisible by P
// https://leetcode.com/problems/make-sum-divisible-by-p/

struct Solution;

impl Solution {
    pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::make_sum_divisible_by_p::Solution;

    #[test]
    fn test_min_subarray_1() {
        let nums = [3, 1, 4, 2].to_vec();
        let p = 6;
        assert_eq!(1, Solution::min_subarray(nums, p));
    }

    #[test]
    fn test_min_subarray_2() {
        let nums = [6, 3, 5, 2].to_vec();
        let p = 9;
        assert_eq!(2, Solution::min_subarray(nums, p));
    }

    #[test]
    fn test_min_subarray_3() {
        let nums = [1, 2, 3].to_vec();
        let p = 3;
        assert_eq!(0, Solution::min_subarray(nums, p));
    }
}
