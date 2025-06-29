// # 1498. Number of Subsequences That Satisfy the Given Sum Condition
// https://leetcode.com/problems/number-of-subsequences-that-satisfy-the-given-sum-condition/

struct Solution;
impl Solution {
    pub fn num_subseq(nums: Vec<i32>, target: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::number_of_subsequences_that_satisfy_the_given_sum_condition::Solution;

    #[test]
    fn test_num_subseq_1() {
        let nums = vec![3, 5, 6, 7];
        let target = 9;
        assert_eq!(4, Solution::num_subseq(nums, target));
    }
    #[test]
    fn test_num_subseq_2() {
        let nums = vec![3, 3, 6, 8];
        let target = 10;
        assert_eq!(6, Solution::num_subseq(nums, target));
    }
    #[test]
    fn test_num_subseq_3() {
        let nums = vec![2, 3, 3, 4, 6, 7];
        let target = 12;
        assert_eq!(61, Solution::num_subseq(nums, target));
    }
}
