// 3634. Minimum Removals to Balance Array
// https://leetcode.com/problems/minimum-removals-to-balance-array/

struct Solution;

impl Solution {
    // https://leetcode.com/problems/minimum-removals-to-balance-array/editorial/
    pub fn min_removal(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut nums = nums;
        nums.sort();

        let mut ans = n as i32;
        let mut right = 0;

        for left in 0..n {
            while right < n && (nums[right] as i64) <= (nums[left] as i64) * (k as i64) {
                right += 1;
            }
            ans = ans.min(n as i32 - (right - left) as i32);
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::minimum_removals_to_balance_array::Solution;

    #[test]
    fn test_min_removal_1() {
        let nums = [2, 1, 5].to_vec();
        let k = 2;
        assert_eq!(1, Solution::min_removal(nums, k));
    }

    #[test]
    fn test_min_removal_2() {
        let nums = [1, 6, 2, 9].to_vec();
        let k = 3;
        assert_eq!(2, Solution::min_removal(nums, k));
    }

    #[test]
    fn test_min_removal_3() {
        let nums = [4, 6].to_vec();
        let k = 2;
        assert_eq!(0, Solution::min_removal(nums, k));
    }

    #[test]
    fn test_min_removal_4() {
        let nums = [12, 18].to_vec();
        let k = 2;
        assert_eq!(0, Solution::min_removal(nums, k));
    }

    #[test]
    fn test_min_removal_5() {
        let nums = [2, 12].to_vec();
        let k = 2;
        assert_eq!(1, Solution::min_removal(nums, k));
    }

    #[test]
    fn test_min_removal_6() {
        let nums = [16, 18].to_vec();
        let k = 1;
        assert_eq!(1, Solution::min_removal(nums, k));
    }

    #[test]
    fn test_min_removal_7() {
        let nums = [11, 16].to_vec();
        let k = 16156;
        assert_eq!(0, Solution::min_removal(nums, k));
    }

    #[test]
    fn test_min_removal_8() {
        let nums = [7, 1].to_vec();
        let k = 2;
        assert_eq!(1, Solution::min_removal(nums, k));
    }
}
