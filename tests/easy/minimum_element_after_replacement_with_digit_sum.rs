// 3300. Minimum Element After Replacement With Digit Sum
// https://leetcode.com/problems/minimum-element-after-replacement-with-digit-sum/

struct Solution;

impl Solution {
    pub fn min_element(nums: Vec<i32>) -> i32 {
        let mut min = i32::MAX;
        for n in nums {
            let mut n = n;
            let mut v = n % 10;
            while n > 9 {
                n = n / 10;
                v = v + n % 10;
            }
            min = min.min(v);
        }
        min
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::minimum_element_after_replacement_with_digit_sum::Solution;

    #[test]
    fn test_min_element_1() {
        let nums = [10, 12, 13, 14].to_vec();
        assert_eq!(1, Solution::min_element(nums));
    }

    #[test]
    fn test_min_element_2() {
        let nums = [1, 2, 3, 4].to_vec();
        assert_eq!(1, Solution::min_element(nums));
    }

    #[test]
    fn test_min_element_3() {
        let nums = [999, 19, 199].to_vec();
        assert_eq!(10, Solution::min_element(nums));
    }
}
