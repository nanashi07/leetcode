// 3471. Find the Largest Almost Missing Integer
// https://leetcode.com/problems/find-the-largest-almost-missing-integer/

struct Solution;

impl Solution {
    pub fn largest_integer(nums: Vec<i32>, k: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::find_the_largest_almost_missing_integer::Solution;

    #[test]
    fn test_largest_integer_1() {
        let nums = [3, 9, 2, 1, 7].to_vec();
        let k = 3;
        assert_eq!(7, Solution::largest_integer(nums, k));
    }

    #[test]
    fn test_largest_integer_2() {
        let nums = [3, 9, 7, 2, 1, 7].to_vec();
        let k = 4;
        assert_eq!(3, Solution::largest_integer(nums, k));
    }

    #[test]
    fn test_largest_integer_3() {
        let nums = [0, 0].to_vec();
        let k = 1;
        assert_eq!(-1, Solution::largest_integer(nums, k));
    }
}
