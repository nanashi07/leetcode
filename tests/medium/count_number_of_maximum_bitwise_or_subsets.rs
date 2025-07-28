// # 2044. Count Number of Maximum Bitwise-OR Subsets
// https://leetcode.com/problems/count-number-of-maximum-bitwise-or-subsets/

struct Solution;

impl Solution {
    pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::count_number_of_maximum_bitwise_or_subsets::Solution;

    #[test]
    fn test_count_max_or_subsets_1() {
        let nums = [3, 1].to_vec();
        assert_eq!(2, Solution::count_max_or_subsets(nums));
    }

    #[test]
    fn test_count_max_or_subsets_2() {
        let nums = [2, 2, 2].to_vec();
        assert_eq!(7, Solution::count_max_or_subsets(nums));
    }

    #[test]
    fn test_count_max_or_subsets_3() {
        let nums = [3, 2, 1, 5].to_vec();
        assert_eq!(6, Solution::count_max_or_subsets(nums));
    }
}
