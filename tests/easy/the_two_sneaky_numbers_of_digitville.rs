// # 3289. The Two Sneaky Numbers of Digitville
// https://leetcode.com/problems/the-two-sneaky-numbers-of-digitville/

use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn get_sneaky_numbers(nums: Vec<i32>) -> Vec<i32> {
        println!("nums: {:?}", &nums);

        let mut set = HashSet::new();
        let mut extra = vec![];

        for n in nums {
            if !set.insert(n) {
                extra.push(n);
            }
        }

        extra
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::the_two_sneaky_numbers_of_digitville::Solution;

    #[test]
    fn test_get_sneaky_numbers_1() {
        let nums = [0, 1, 1, 0].to_vec();
        let mut target = [0, 1].to_vec();
        let mut result = Solution::get_sneaky_numbers(nums);
        target.sort_unstable();
        result.sort_unstable();
        assert_eq!(target, result);
    }

    #[test]
    fn test_get_sneaky_numbers_2() {
        let nums = [0, 3, 2, 1, 3, 2].to_vec();
        let mut target = [2, 3].to_vec();
        let mut result = Solution::get_sneaky_numbers(nums);
        target.sort_unstable();
        result.sort_unstable();
        assert_eq!(target, result);
    }

    #[test]
    fn test_get_sneaky_numbers_31() {
        let nums = [7, 1, 5, 4, 3, 4, 6, 0, 9, 5, 8, 2].to_vec();
        let mut target = [4, 5].to_vec();
        let mut result = Solution::get_sneaky_numbers(nums);
        target.sort_unstable();
        result.sort_unstable();
        assert_eq!(target, result);
    }
}
