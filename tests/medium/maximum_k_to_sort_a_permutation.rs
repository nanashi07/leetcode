// 3644. Maximum K to Sort a Permutation
// https://leetcode.com/problems/maximum-k-to-sort-a-permutation/

struct Solution;

impl Solution {
    pub fn sort_permutation(nums: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::maximum_k_to_sort_a_permutation::Solution;

    #[test]
    fn test_sort_permutation_1() {
        let nums = [0, 3, 2, 1].to_vec();
        assert_eq!(1, Solution::sort_permutation(nums));
    }

    #[test]
    fn test_sort_permutation_2() {
        let nums = [0, 1, 3, 2].to_vec();
        assert_eq!(2, Solution::sort_permutation(nums));
    }

    #[test]
    fn test_sort_permutation_3() {
        let nums = [3, 2, 1, 0].to_vec();
        assert_eq!(0, Solution::sort_permutation(nums));
    }
}
