// 1464. Maximum Product of Two Elements in an Array
// https://leetcode.com/problems/maximum-product-of-two-elements-in-an-array/

struct Solution;

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        nums.iter().rev().take(2).map(|n| n - 1).product::<i32>()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::maximum_product_of_two_elements_in_an_array::Solution;

    #[test]
    fn test_max_product_1() {
        let nums = [3, 4, 5, 2].to_vec();
        assert_eq!(12, Solution::max_product(nums));
    }

    #[test]
    fn test_max_product_2() {
        let nums = [1, 5, 4, 5].to_vec();
        assert_eq!(16, Solution::max_product(nums));
    }

    #[test]
    fn test_max_product_3() {
        let nums = [3, 7].to_vec();
        assert_eq!(12, Solution::max_product(nums));
    }
}
