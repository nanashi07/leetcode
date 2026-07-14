// 3336. Find the Number of Subsequences With Equal GCD
// https://leetcode.com/problems/find-the-number-of-subsequences-with-equal-gcd/

struct Solution;

impl Solution {
    pub fn subsequence_pair_count(nums: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::find_the_number_of_subsequences_with_equal_gcd::Solution;

    #[test]
    fn test_subsequence_pair_count_1() {
        let nums = [1, 2, 3, 4].to_vec();
        assert_eq!(10, Solution::subsequence_pair_count(nums));
    }

    #[test]
    fn test_subsequence_pair_count_2() {
        let nums = [10, 20, 30].to_vec();
        assert_eq!(2, Solution::subsequence_pair_count(nums));
    }

    #[test]
    fn test_subsequence_pair_count_3() {
        let nums = [1, 1, 1, 1].to_vec();
        assert_eq!(50, Solution::subsequence_pair_count(nums));
    }
}
