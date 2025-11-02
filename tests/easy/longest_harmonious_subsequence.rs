// 594. Longest Harmonious Subsequence
// https://leetcode.com/problems/longest-harmonious-subsequence/

struct Solution;
impl Solution {
    // best code sample from leetcode submissions
    pub fn _best_find_lhs(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let mut max = 0;
        let mut start = 0;

        for end in 1..nums.len() {
            while nums[end] - nums[start] > 1 {
                start += 1;
            }
            if nums[end] - nums[start] == 1 {
                max = max.max(end - start + 1);
            }
        }

        max as i32
    }

    // my submission
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let min = nums[0];
        let max = nums[nums.len() - 1];

        let mut count = vec![0; (max - min + 1) as usize];
        for i in 0..nums.len() {
            count[(nums[i] - min) as usize] += 1;
        }

        let mut max_len = 0;
        for i in 0..count.len() - 1 {
            if count[i] > 0 && count[i + 1] > 0 && count[i] + count[i + 1] > max_len {
                max_len = count[i] + count[i + 1];
            }
        }

        max_len
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::longest_harmonious_subsequence::Solution;

    #[test]
    fn test_find_lhs_1() {
        let nums = [1, 3, 2, 2, 5, 2, 3, 7].to_vec();
        assert_eq!(5, Solution::find_lhs(nums));
    }

    #[test]
    fn test_find_lhs_2() {
        let nums = [1, 2, 3, 4].to_vec();
        assert_eq!(2, Solution::find_lhs(nums));
    }

    #[test]
    fn test_find_lhs_3() {
        let nums = [1, 1, 1, 1].to_vec();
        assert_eq!(0, Solution::find_lhs(nums));
    }

    #[test]
    fn test_find_lhs_4() {
        let nums = [1, 3, 5, 7, 9, 11, 13, 15, 17].to_vec();
        assert_eq!(0, Solution::find_lhs(nums));
    }
}
