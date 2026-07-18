// 1979. Find Greatest Common Divisor of Array
// https://leetcode.com/problems/find-greatest-common-divisor-of-array/

struct Solution;

impl Solution {
    pub fn find_gcd(nums: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::find_greatest_common_divisor_of_array::Solution;

    #[test]
    fn test_find_gcd_1() {
        let nums = [2, 5, 6, 9, 10].to_vec();
        assert_eq!(2, Solution::find_gcd(nums));
    }

    #[test]
    fn test_find_gcd_2() {
        let nums = [7, 5, 6, 8, 3].to_vec();
        assert_eq!(1, Solution::find_gcd(nums));
    }

    #[test]
    fn test_find_gcd_3() {
        let nums = [3, 3].to_vec();
        assert_eq!(3, Solution::find_gcd(nums));
    }
}
