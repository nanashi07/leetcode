// 2567. Minimum Score by Changing Two Elements
// https://leetcode.com/problems/minimum-score-by-changing-two-elements/

struct Solution;

impl Solution {
    pub fn minimize_sum(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        // With at most 3 elements we can make everything equal to one kept value.
        if n <= 3 {
            return 0;
        }

        // The score is max - min, so the two changed elements should swallow the
        // extremes: drop the two smallest, the two largest, or one of each.
        nums.sort_unstable();
        let keep_lowest = nums[n - 3] - nums[0];
        let keep_highest = nums[n - 1] - nums[2];
        let keep_middle = nums[n - 2] - nums[1];

        keep_lowest.min(keep_highest).min(keep_middle)
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::minimum_score_by_changing_two_elements::Solution;

    #[test]
    fn test_minimize_sum_1() {
        let nums: Vec<i32> = vec![1, 4, 7, 8, 5];
        assert_eq!(3, Solution::minimize_sum(nums));
    }

    #[test]
    fn test_minimize_sum_2() {
        let nums: Vec<i32> = vec![1, 4, 3];
        assert_eq!(0, Solution::minimize_sum(nums));
    }
}
