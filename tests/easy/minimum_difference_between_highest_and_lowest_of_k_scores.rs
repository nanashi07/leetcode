// 1984. Minimum Difference Between Highest and Lowest of K Scores
// https://leetcode.com/problems/minimum-difference-between-highest-and-lowest-of-k-scores/

struct Solution;

impl Solution {
    pub fn minimum_difference(nums: Vec<i32>, k: i32) -> i32 {
        print!("nums: {nums:?}, k: {k}");

        let k = k as usize;
        let mut nums = nums;
        nums.sort_unstable();

        if nums.len() <= k {
            return nums[nums.len() - 1] - nums[0];
        }

        nums.iter()
            .take(nums.len() - k + 1)
            .enumerate()
            .map(|(i, n)| nums[i + k - 1] - *n)
            .min()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::minimum_difference_between_highest_and_lowest_of_k_scores::Solution;

    #[test]
    fn test_minimum_difference_1() {
        let nums = [90].to_vec();
        let k = 1;
        assert_eq!(0, Solution::minimum_difference(nums, k));
    }

    #[test]
    fn test_minimum_difference_2() {
        let nums = [9, 4, 1, 7].to_vec();
        let k = 2;
        assert_eq!(2, Solution::minimum_difference(nums, k));
    }
}
