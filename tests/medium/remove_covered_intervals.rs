// 1288. Remove Covered Intervals
// https://leetcode.com/problems/remove-covered-intervals/

struct Solution;

impl Solution {
    pub fn remove_covered_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::remove_covered_intervals::Solution;
    use crate::shared::vec2d::to_vec2d;

    #[test]
    fn test_remove_covered_intervals_1() {
        let intervals = to_vec2d([[1, 4], [3, 6], [2, 8]]);
        assert_eq!(2, Solution::remove_covered_intervals(intervals));
    }

    #[test]
    fn test_remove_covered_intervals_2() {
        let intervals = to_vec2d([[1, 4], [2, 3]]);
        assert_eq!(1, Solution::remove_covered_intervals(intervals));
    }
}
