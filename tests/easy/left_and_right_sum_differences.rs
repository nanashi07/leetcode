// 2574. Left and Right Sum Differences
// https://leetcode.com/problems/left-and-right-sum-differences/

struct Solution;

impl Solution {
    pub fn left_right_difference(nums: Vec<i32>) -> Vec<i32> {
        let mut answer = vec![];
        let mut l = 0;
        let mut r = nums.iter().sum::<i32>();
        for i in 0..nums.len() {
            l = l + if i > 0 { nums[i - 1] } else { 0 };
            r = r - nums[i];
            answer.push((l - r).abs());
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::left_and_right_sum_differences::Solution;

    #[test]
    fn test_left_right_difference_1() {
        let nums = [10, 4, 8, 3].to_vec();
        assert_eq!(
            [15, 1, 11, 22].to_vec(),
            Solution::left_right_difference(nums)
        );
    }

    #[test]
    fn test_left_right_difference_2() {
        let nums = [1].to_vec();
        assert_eq!([0].to_vec(), Solution::left_right_difference(nums));
    }
}
