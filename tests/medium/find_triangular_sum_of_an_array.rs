// # 2221. Find Triangular Sum of an Array
// https://leetcode.com/problems/find-triangular-sum-of-an-array/

struct Solution;

impl Solution {
    pub fn triangular_sum(nums: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::find_triangular_sum_of_an_array::Solution;

    #[test]
    fn test_triangular_sum_1() {
        let nums = [1, 2, 3, 4, 5].to_vec();
        assert_eq!(8, Solution::triangular_sum(nums));
    }

    #[test]
    fn test_triangular_sum_2() {
        let nums = [5].to_vec();
        assert_eq!(5, Solution::triangular_sum(nums));
    }
}
