// 2091. Removing Minimum and Maximum From Array
// https://leetcode.com/problems/removing-minimum-and-maximum-from-array/

struct Solution;

impl Solution {
    pub fn minimum_deletions(nums: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::removing_minimum_and_maximum_from_array::Solution;

    #[test]
    fn test_minimum_deletions_1() {
        let nums = [2, 10, 7, 5, 4, 1, 8, 6].to_vec();
        assert_eq!(5, Solution::minimum_deletions(nums));
    }

    #[test]
    fn test_minimum_deletions_2() {
        let nums = [0, -4, 19, 1, 8, -2, -3, 5].to_vec();
        assert_eq!(3, Solution::minimum_deletions(nums));
    }

    #[test]
    fn test_minimum_deletions_3() {
        let nums = [101].to_vec();
        assert_eq!(1, Solution::minimum_deletions(nums));
    }
}
