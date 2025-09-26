// # 611. Valid Triangle Number
// https://leetcode.com/problems/valid-triangle-number/

struct Solution;

impl Solution {
    pub fn triangle_number(nums: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::valid_triangle_number::Solution;

    #[test]
    fn test_triangle_number_1() {
        let nums = [2, 2, 3, 4].to_vec();
        assert_eq!(3, Solution::triangle_number(nums));
    }

    #[test]
    fn test_triangle_number_2() {
        let nums = [4, 2, 3, 4].to_vec();
        assert_eq!(4, Solution::triangle_number(nums));
    }
}
