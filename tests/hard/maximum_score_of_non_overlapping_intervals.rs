// 3414. Maximum Score of Non-overlapping Intervals
// https://leetcode.com/problems/maximum-score-of-non-overlapping-intervals/

struct Solution;

impl Solution {
    pub fn maximum_weight(intervals: Vec<Vec<i32>>) -> Vec<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::maximum_score_of_non_overlapping_intervals::Solution;
    use crate::shared::vec2d::to_vec2d;

    #[test]
    fn test_maximum_weight_1() {
        let intervals = to_vec2d([
            [1, 3, 2],
            [4, 5, 2],
            [1, 5, 5],
            [6, 9, 3],
            [6, 7, 1],
            [8, 9, 1],
        ]);
        assert_eq!([2, 3].to_vec(), Solution::maximum_weight(intervals));
    }

    #[test]
    fn test_maximum_weight_2() {
        let intervals = to_vec2d([
            [5, 8, 1],
            [6, 7, 7],
            [4, 7, 3],
            [9, 10, 6],
            [7, 8, 2],
            [11, 14, 3],
            [3, 5, 5],
        ]);
        assert_eq!([1, 3, 5, 6].to_vec(), Solution::maximum_weight(intervals));
    }
}
