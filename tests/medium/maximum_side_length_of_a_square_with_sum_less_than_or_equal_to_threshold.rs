// 1292. Maximum Side Length of a Square with Sum Less than or Equal to Threshold
// https://leetcode.com/problems/maximum-side-length-of-a-square-with-sum-less-than-or-equal-to-threshold/

struct Solution;

impl Solution {
    pub fn max_side_length(mat: Vec<Vec<i32>>, threshold: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::maximum_side_length_of_a_square_with_sum_less_than_or_equal_to_threshold::Solution;

    #[test]
    fn test_max_side_length_1() {
        let mat = [
            [1, 1, 3, 2, 4, 3, 2],
            [1, 1, 3, 2, 4, 3, 2],
            [1, 1, 3, 2, 4, 3, 2],
        ]
        .iter()
        .map(|l| l.to_vec())
        .collect::<Vec<_>>();
        let threshold = 4;
        assert_eq!(2, Solution::max_side_length(mat, threshold));
    }

    #[test]
    fn test_max_side_length_2() {
        let mat = [
            [2, 2, 2, 2, 2],
            [2, 2, 2, 2, 2],
            [2, 2, 2, 2, 2],
            [2, 2, 2, 2, 2],
            [2, 2, 2, 2, 2],
        ]
        .iter()
        .map(|l| l.to_vec())
        .collect::<Vec<_>>();
        let threshold = 1;
        assert_eq!(0, Solution::max_side_length(mat, threshold));
    }
}
