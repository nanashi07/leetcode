// 2492. Minimum Score of a Path Between Two Cities
// https://leetcode.com/problems/minimum-score-of-a-path-between-two-cities/

struct Solution;

impl Solution {
    pub fn min_score(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::minimum_score_of_a_path_between_two_cities::Solution;
    use crate::shared::vec2d::to_vec2d;

    #[test]
    fn test_min_score_1() {
        let n = 4;
        let roads = to_vec2d([[1, 2, 9], [2, 3, 6], [2, 4, 5], [1, 4, 7]]);
        assert_eq!(5, Solution::min_score(n, roads));
    }

    #[test]
    fn test_min_score_2() {
        let n = 4;
        let roads = to_vec2d([[1, 2, 2], [1, 3, 4], [3, 4, 7]]);
        assert_eq!(2, Solution::min_score(n, roads));
    }
}
