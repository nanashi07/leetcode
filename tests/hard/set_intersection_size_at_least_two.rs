// 757. Set Intersection Size At Least Two
// https://leetcode.com/problems/set-intersection-size-at-least-two/

struct Solution;

impl Solution {
    pub fn intersection_size_two(intervals: Vec<Vec<i32>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::set_intersection_size_at_least_two::Solution;

    #[test]
    fn test_intersection_size_two_1() {
        let intervals = [[1, 3], [3, 7], [8, 9]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(5, Solution::intersection_size_two(intervals));
    }

    #[test]
    fn test_intersection_size_two_2() {
        let intervals = [[1, 3], [1, 4], [2, 5], [3, 5]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(3, Solution::intersection_size_two(intervals));
    }

    #[test]
    fn test_intersection_size_two_3() {
        let intervals = [[1, 2], [2, 3], [2, 4], [4, 5]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(5, Solution::intersection_size_two(intervals));
    }
}
