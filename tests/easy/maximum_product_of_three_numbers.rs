// 628. Maximum Product of Three Numbers
// https://leetcode.com/problems/maximum-product-of-three-numbers/

struct Solution;

impl Solution {
    pub fn maximum_product(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let c = nums.iter().filter(|&n| *n < 0).count();
        let r = nums.iter().rev().take(3).product::<i32>();
        if c > 1 {
            let l = nums.iter().take(2).product::<i32>() * nums[nums.len() - 1];
            if r > l {
                r
            } else {
                l
            }
        } else {
            r
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::maximum_product_of_three_numbers::Solution;

    #[test]
    fn test_maximum_product_1() {
        let nums = [1, 2, 3].to_vec();
        assert_eq!(6, Solution::maximum_product(nums));
    }

    #[test]
    fn test_maximum_product_2() {
        let nums = [1, 2, 3, 4].to_vec();
        assert_eq!(24, Solution::maximum_product(nums));
    }

    #[test]
    fn test_maximum_product_3() {
        let nums = [-1, -2, -3].to_vec();
        assert_eq!(-6, Solution::maximum_product(nums));
    }

    #[test]
    fn test_maximum_product_4() {
        let nums = [-100, -98, -1, 2, 3, 4].to_vec();
        assert_eq!(39200, Solution::maximum_product(nums));
    }
}
