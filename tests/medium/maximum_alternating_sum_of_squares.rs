// 3727. Maximum Alternating Sum of Squares
// https://leetcode.com/problems/maximum-alternating-sum-of-squares/

struct Solution;

impl Solution {
    pub fn max_alternating_sum(nums: Vec<i32>) -> i64 {
        let mut squares: Vec<i64> = nums.into_iter().map(|num| i64::from(num).pow(2)).collect();
        let negative_count = squares.len() / 2;

        if negative_count == 0 {
            return squares.iter().sum();
        }

        squares.select_nth_unstable(negative_count);
        let negative_sum: i64 = squares[..negative_count].iter().sum();
        let total_sum: i64 = squares.iter().sum();

        total_sum - 2 * negative_sum
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::maximum_alternating_sum_of_squares::Solution;

    #[test]
    fn test_max_alternating_sum_1() {
        let nums = [1, 2, 3].to_vec();
        assert_eq!(12, Solution::max_alternating_sum(nums));
    }

    #[test]
    fn test_max_alternating_sum_2() {
        let nums = [1, -1, 2, -2, 3, -3].to_vec();
        assert_eq!(16, Solution::max_alternating_sum(nums));
    }
}
