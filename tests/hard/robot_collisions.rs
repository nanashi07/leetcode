// 2751. Robot Collisions
// https://leetcode.com/problems/robot-collisions/

struct Solution;

impl Solution {
    pub fn survived_robots_healths(
        positions: Vec<i32>,
        healths: Vec<i32>,
        directions: String,
    ) -> Vec<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::robot_collisions::Solution;

    #[test]
    fn test_survived_robots_healths_1() {
        let positions = [5, 4, 3, 2, 1].to_vec();
        let healths = [2, 17, 9, 15, 10].to_vec();
        let directions = "RRRRR".to_string();
        assert_eq!(
            [2, 17, 9, 15, 10].to_vec(),
            Solution::survived_robots_healths(positions, healths, directions)
        );
    }

    #[test]
    fn test_survived_robots_healths_2() {
        let positions = [3, 5, 2, 6].to_vec();
        let healths = [10, 10, 15, 12].to_vec();
        let directions = "RLRL".to_string();
        assert_eq!(
            [14].to_vec(),
            Solution::survived_robots_healths(positions, healths, directions)
        );
    }

    #[test]
    fn test_survived_robots_healths_3() {
        let positions = [1, 2, 5, 6].to_vec();
        let healths = [10, 10, 11, 11].to_vec();
        let directions = "RLRL".to_string();
        assert_eq!(
            [0; 0].to_vec(),
            Solution::survived_robots_healths(positions, healths, directions)
        );
    }
}
