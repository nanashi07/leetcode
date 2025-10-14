// # 3349. Adjacent Increasing Subarrays Detection I
// https://leetcode.com/problems/adjacent-increasing-subarrays-detection-i/

struct Solution;

impl Solution {
    pub fn has_increasing_subarrays(nums: Vec<i32>, k: i32) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::adjacent_increasing_subarrays_detection_i::Solution;

    #[test]
    fn test_has_increasing_subarrays_1() {
        let nums = [2, 5, 7, 8, 9, 2, 3, 4, 3, 1].to_vec();
        let k = 3;
        assert_eq!(true, Solution::has_increasing_subarrays(nums, k));
    }

    #[test]
    fn test_has_increasing_subarrays_2() {
        let nums = [1, 2, 3, 4, 4, 4, 4, 5, 6, 7].to_vec();
        let k = 5;
        assert_eq!(false, Solution::has_increasing_subarrays(nums, k));
    }
}
