// 3661. Maximum Walls Destroyed by Robots
// https://leetcode.com/problems/maximum-walls-destroyed-by-robots/

struct Solution;

impl Solution {
    pub fn max_walls(robots: Vec<i32>, distance: Vec<i32>, walls: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::maximum_walls_destroyed_by_robots::Solution;

    #[test]
    fn test_max_walls_1() {
        let robots = [4].to_vec();
        let distance = [3].to_vec();
        let walls = [1, 10].to_vec();
        assert_eq!(1, Solution::max_walls(robots, distance, walls));
    }

    #[test]
    fn test_max_walls_2() {
        let robots = [10, 2].to_vec();
        let distance = [5, 1].to_vec();
        let walls = [5, 2, 7].to_vec();
        assert_eq!(3, Solution::max_walls(robots, distance, walls));
    }

    #[test]
    fn test_max_walls_3() {
        let robots = [1, 2].to_vec();
        let distance = [100, 1].to_vec();
        let walls = [10].to_vec();
        assert_eq!(0, Solution::max_walls(robots, distance, walls));
    }
}
