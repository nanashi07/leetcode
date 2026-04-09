// 3655. XOR After Range Multiplication Queries II
// https://leetcode.com/problems/xor-after-range-multiplication-queries-ii/

struct Solution;

impl Solution {
    pub fn xor_after_queries(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::xor_after_range_multiplication_queries_ii::Solution;
    use crate::shared::vec2d::to_vec2d;

    #[test]
    fn test_xor_after_queries_1() {
        let nums = [1, 1, 1].to_vec();
        let queries = to_vec2d([[0, 2, 1, 4]]);
        assert_eq!(4, Solution::xor_after_queries(nums, queries));
    }

    #[test]
    fn test_xor_after_queries_2() {
        let nums = [2, 3, 1, 5, 4].to_vec();
        let queries = to_vec2d([[1, 4, 2, 3], [0, 2, 1, 2]]);
        assert_eq!(31, Solution::xor_after_queries(nums, queries));
    }
}
