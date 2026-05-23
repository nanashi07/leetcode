// 1752. Check if Array Is Sorted and Rotated
// https://leetcode.com/problems/check-if-array-is-sorted-and-rotated/

struct Solution;

impl Solution {
    pub fn check(nums: Vec<i32>) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::check_if_array_is_sorted_and_rotated::Solution;

    #[test]
    fn test_check_1() {
        let nums = [3, 4, 5, 1, 2].to_vec();
        assert_eq!(true, Solution::check(nums));
    }

    #[test]
    fn test_check_2() {
        let nums = [2, 1, 3, 4].to_vec();
        assert_eq!(false, Solution::check(nums));
    }

    #[test]
    fn test_check_3() {
        let nums = [1, 2, 3].to_vec();
        assert_eq!(true, Solution::check(nums));
    }
}
