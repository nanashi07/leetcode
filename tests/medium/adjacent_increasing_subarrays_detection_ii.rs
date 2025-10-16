// # 3350. Adjacent Increasing Subarrays Detection II
// https://leetcode.com/problems/adjacent-increasing-subarrays-detection-ii/

struct Solution;

impl Solution {
    pub fn max_increasing_subarrays(nums: Vec<i32>) -> i32 {
        println!("nums: {:?}", &nums);
        let n = nums.len();
        let mut cnt = 1;
        let mut precnt = 0;
        let mut ans = 0;

        for i in 1..n {
            if nums[i] > nums[i - 1] {
                cnt += 1;
            } else {
                precnt = cnt;
                cnt = 1;
            }
            ans = ans.max(precnt.min(cnt));
            ans = ans.max(cnt / 2);
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::adjacent_increasing_subarrays_detection_ii::Solution;

    #[test]
    fn test_max_increasing_subarrays_1() {
        let nums = [2, 5, 7, 8, 9, 2, 3, 4, 3, 1].to_vec();
        assert_eq!(3, Solution::max_increasing_subarrays(nums));
    }

    #[test]
    fn test_max_increasing_subarrays_2() {
        let nums = [1, 2, 3, 4, 4, 4, 4, 5, 6, 7].to_vec();
        assert_eq!(2, Solution::max_increasing_subarrays(nums));
    }
}
