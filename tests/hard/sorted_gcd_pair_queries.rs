// 3312. Sorted GCD Pair Queries
// https://leetcode.com/problems/sorted-gcd-pair-queries/

struct Solution;

impl Solution {
    pub fn gcd_values(nums: Vec<i32>, queries: Vec<i64>) -> Vec<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::sorted_gcd_pair_queries::Solution;

    #[test]
    fn test_gcd_values_1() {
        let nums = [2, 3, 4].to_vec();
        let queries = [0, 2, 2].to_vec();
        assert_eq!([1, 2, 2].to_vec(), Solution::gcd_values(nums, queries));
    }

    #[test]
    fn test_gcd_values_2() {
        let nums = [4, 4, 2, 1].to_vec();
        let queries = [5, 3, 1, 0].to_vec();
        assert_eq!([4, 2, 1, 1].to_vec(), Solution::gcd_values(nums, queries));
    }

    #[test]
    fn test_gcd_values_3() {
        let nums = [2, 2].to_vec();
        let queries = [0, 0].to_vec();
        assert_eq!([2, 2].to_vec(), Solution::gcd_values(nums, queries));
    }
}
