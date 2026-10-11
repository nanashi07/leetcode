// 2778. Sum of Squares of Special Elements
// https://leetcode.com/problems/sum-of-squares-of-special-elements/

struct Solution;

impl Solution {
    pub fn sum_of_squares(nums: Vec<i32>) -> i32 {
        let mut s = 0;
        for (i, &n) in nums.iter().enumerate() {
            if nums.len() % (i + 1) == 0 {
                s += n * n;
            }
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::sum_of_squares_of_special_elements::Solution;

    #[test]
    fn test_sum_of_squares_1() {
        let nums = [1, 2, 3, 4].to_vec();
        assert_eq!(21, Solution::sum_of_squares(nums));
    }

    #[test]
    fn test_sum_of_squares_2() {
        let nums = [2, 7, 1, 19, 18, 3].to_vec();
        assert_eq!(63, Solution::sum_of_squares(nums));
    }
}
