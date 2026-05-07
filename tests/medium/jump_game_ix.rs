// 3660. Jump Game IX
// https://leetcode.com/problems/jump-game-ix/

struct Solution;

impl Solution {
    pub fn max_value(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut ans = vec![0; n];
        let mut pre_max = vec![nums[0]; n];

        for i in 1..n {
            pre_max[i] = pre_max[i - 1].max(nums[i]);
        }

        let mut suf_min = i32::MAX;

        for i in (0..n).rev() {
            ans[i] = if i == n - 1 || pre_max[i] <= suf_min {
                pre_max[i]
            } else {
                ans[i + 1]
            };
            suf_min = suf_min.min(nums[i]);
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::jump_game_ix::Solution;

    #[test]
    fn test_max_value_1() {
        let nums = [2, 1, 3].to_vec();
        assert_eq!([2, 2, 3].to_vec(), Solution::max_value(nums));
    }

    #[test]
    fn test_max_value_2() {
        let nums = [2, 3, 1].to_vec();
        assert_eq!([3, 3, 3].to_vec(), Solution::max_value(nums));
    }
}
