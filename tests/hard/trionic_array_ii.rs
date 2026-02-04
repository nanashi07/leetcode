// 3640. Trionic Array II
// https://leetcode.com/problems/trionic-array-ii/

struct Solution;

impl Solution {
    pub fn max_sum_trionic(nums: Vec<i32>) -> i64 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::trionic_array_ii::Solution;

    #[test]
    fn test_max_sum_trionic_1() {
        let nums = [0, -2, -1, -3, 0, 2, -1].to_vec();
        assert_eq!(-4, Solution::max_sum_trionic(nums));
    }

    #[test]
    fn test_max_sum_trionic_2() {
        let nums = [1, 4, 2, 7].to_vec();
        assert_eq!(14, Solution::max_sum_trionic(nums));
    }
}
