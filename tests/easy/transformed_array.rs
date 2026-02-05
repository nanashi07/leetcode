// 3379. Transformed Array
// https://leetcode.com/problems/transformed-array/

struct Solution;

impl Solution {
    pub fn construct_transformed_array(nums: Vec<i32>) -> Vec<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::transformed_array::Solution;

    #[test]
    fn test_construct_transformed_array_1() {
        let nums = [3, -2, 1, 1].to_vec();
        let output = [1, 1, 1, 3].to_vec();
        assert_eq!(output, Solution::construct_transformed_array(nums));
    }

    #[test]
    fn test_construct_transformed_array_2() {
        let nums = [-1, 4, -1].to_vec();
        let output = [-1, -1, 4].to_vec();
        assert_eq!(output, Solution::construct_transformed_array(nums));
    }
}
