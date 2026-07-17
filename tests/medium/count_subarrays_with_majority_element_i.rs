// 3737. Count Subarrays With Majority Element I
// https://leetcode.com/problems/count-subarrays-with-majority-element-i/

struct Solution;

impl Solution {
    pub fn count_majority_subarrays(nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();
        let mut count = 0;
        for i in 0..n {
            let mut target_count = 0;
            for (j, _) in nums.iter().enumerate().take(n).skip(i) {
                if nums[j] == target {
                    target_count += 1;
                }
                let len = j - i + 1;
                if target_count * 2 > len {
                    count += 1;
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::count_subarrays_with_majority_element_i::Solution;

    #[test]
    fn test_count_majority_subarrays_1() {
        let nums = [1, 2, 2, 3].to_vec();
        let target = 2;
        assert_eq!(5, Solution::count_majority_subarrays(nums, target));
    }

    #[test]
    fn test_count_majority_subarrays_2() {
        let nums = [1, 2, 3].to_vec();
        let target = 4;
        assert_eq!(0, Solution::count_majority_subarrays(nums, target));
    }
}
