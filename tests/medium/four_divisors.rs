// 1390. Four Divisors
// https://leetcode.com/problems/four-divisors/

struct Solution;

impl Solution {
    pub fn sum_four_divisors(nums: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::four_divisors::Solution;

    #[test]
    fn test_sum_four_divisors_1() {
        let nums = [21, 4, 7].to_vec();
        assert_eq!(32, Solution::sum_four_divisors(nums));
    }

    #[test]
    fn test_sum_four_divisors_2() {
        let nums = [21, 21].to_vec();
        assert_eq!(64, Solution::sum_four_divisors(nums));
    }

    #[test]
    fn test_sum_four_divisors_3() {
        let nums = [1, 2, 3, 4, 5].to_vec();
        assert_eq!(0, Solution::sum_four_divisors(nums));
    }
}
