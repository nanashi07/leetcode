// 2463. Minimum Total Distance Traveled
// https://leetcode.com/problems/minimum-total-distance-traveled/

struct Solution;

impl Solution {
    pub fn minimum_total_distance(robot: Vec<i32>, factory: Vec<Vec<i32>>) -> i64 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::minimum_total_distance_traveled::Solution;
    use crate::shared::vec2d::to_vec2d;

    #[test]
    fn test_minimum_total_distance_1() {
        let robot = [0, 4, 6].to_vec();
        let factory = to_vec2d([[2, 2], [6, 2]]);
        assert_eq!(4, Solution::minimum_total_distance(robot, factory));
    }

    #[test]
    fn test_minimum_total_distance_2() {
        let robot = [1, -1].to_vec();
        let factory = to_vec2d([[-2, 1], [2, 1]]);
        assert_eq!(2, Solution::minimum_total_distance(robot, factory));
    }
}
