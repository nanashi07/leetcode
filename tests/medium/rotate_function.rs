// 396. Rotate Function
// https://leetcode.com/problems/rotate-function/

struct Solution;

impl Solution {
    pub fn max_rotate_function(nums: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::rotate_function::Solution;

    #[test]
    fn test_max_rotate_function_1() {
        let nums = [4, 3, 2, 6].to_vec();
        assert_eq!(26, Solution::max_rotate_function(nums));
    }

    #[test]
    fn test_max_rotate_function_2() {
        let nums = [100].to_vec();
        assert_eq!(0, Solution::max_rotate_function(nums));
    }
}
