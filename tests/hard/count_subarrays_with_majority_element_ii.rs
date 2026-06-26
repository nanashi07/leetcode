// 3739. Count Subarrays With Majority Element II
// https://leetcode.com/problems/count-subarrays-with-majority-element-ii/

struct Solution;

impl Solution {
    pub fn count_majority_subarrays(nums: Vec<i32>, target: i32) -> i64 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::count_subarrays_with_majority_element_ii::Solution;

    #[test]
    fn test_count_majority_subarrays_1() {
        let nums = [1, 2, 2, 3].to_vec();
        let target = 2;
        assert_eq!(5, Solution::count_majority_subarrays(nums, target));
    }

    #[test]
    fn test_count_majority_subarrays_2() {
        let nums = [1, 1, 1, 1].to_vec();
        let target = 10;
        assert_eq!(1, Solution::count_majority_subarrays(nums, target));
    }

    #[test]
    fn test_count_majority_subarrays_3() {
        let nums = [1, 2, 3].to_vec();
        let target = 4;
        assert_eq!(0, Solution::count_majority_subarrays(nums, target));
    }
}
