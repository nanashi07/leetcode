// 3551. Minimum Swaps to Sort by Digit Sum
// https://leetcode.com/problems/minimum-swaps-to-sort-by-digit-sum/

struct Solution;

impl Solution {
    pub fn min_swaps(nums: Vec<i32>) -> i32 {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::minimum_swaps_to_sort_by_digit_sum::Solution;

    #[test]
    fn test_min_swaps_1() {
        let nums = [37, 100].to_vec();
        assert_eq!(1, Solution::min_swaps(nums));
    }

    #[test]
    fn test_min_swaps_2() {
        let nums = [22, 14, 33, 7].to_vec();
        assert_eq!(0, Solution::min_swaps(nums));
    }

    #[test]
    fn test_min_swaps_3() {
        let nums = [18, 43, 34, 16].to_vec();
        assert_eq!(2, Solution::min_swaps(nums));
    }
}
