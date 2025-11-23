// 1262. Greatest Sum Divisible by Three
// https://leetcode.com/problems/greatest-sum-divisible-by-three/

struct Solution;

impl Solution {
    pub fn max_sum_div_three(nums: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::greatest_sum_divisible_by_three::Solution;

    #[test]
    fn test_max_sum_div_three_1() {
        let nums = [3, 6, 5, 1, 8].to_vec();
        assert_eq!(18, Solution::max_sum_div_three(nums));
    }

    #[test]
    fn test_max_sum_div_three_2() {
        let nums = [4].to_vec();
        assert_eq!(0, Solution::max_sum_div_three(nums));
    }

    #[test]
    fn test_max_sum_div_three_3() {
        let nums = [1, 2, 3, 4, 4].to_vec();
        assert_eq!(12, Solution::max_sum_div_three(nums));
    }
}
