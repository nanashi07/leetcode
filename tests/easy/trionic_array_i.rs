// 3637. Trionic Array I
// https://leetcode.com/problems/trionic-array-i/

struct Solution;

impl Solution {
    pub fn is_trionic(nums: Vec<i32>) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::trionic_array_i::Solution;

    #[test]
    fn test_is_trionic_1() {
        let nums = [1, 3, 5, 4, 2, 6].to_vec();
        assert_eq!(true, Solution::is_trionic(nums));
    }

    #[test]
    fn test_is_trionic_2() {
        let nums = [2, 1, 3].to_vec();
        assert_eq!(false, Solution::is_trionic(nums));
    }
}
