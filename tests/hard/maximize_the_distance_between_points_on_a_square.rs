// 3464. Maximize the Distance Between Points on a Square
// https://leetcode.com/problems/maximize-the-distance-between-points-on-a-square/

struct Solution;

impl Solution {
    pub fn max_distance(side: i32, points: Vec<Vec<i32>>, k: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::maximize_the_distance_between_points_on_a_square::Solution;
    use crate::shared::vec2d::to_vec2d;

    #[test]
    fn test_max_distance_1() {
        let side = 2;
        let points = to_vec2d([[0, 2], [2, 0], [2, 2], [0, 0]]);
        let k = 4;
        assert_eq!(2, Solution::max_distance(side, points, k));
    }

    #[test]
    fn test_max_distance_2() {
        let side = 2;
        let points = to_vec2d([[0, 0], [1, 2], [2, 0], [2, 2], [2, 1]]);
        let k = 4;
        assert_eq!(1, Solution::max_distance(side, points, k));
    }

    #[test]
    fn test_max_distance_3() {
        let side = 2;
        let points = to_vec2d([[0, 0], [0, 1], [0, 2], [1, 2], [2, 0], [2, 2], [2, 1]]);
        let k = 5;
        assert_eq!(1, Solution::max_distance(side, points, k));
    }
}
