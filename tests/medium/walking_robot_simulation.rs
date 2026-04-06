// 874. Walking Robot Simulation
// https://leetcode.com/problems/walking-robot-simulation/

struct Solution;

impl Solution {
    pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::walking_robot_simulation::Solution;
    use crate::shared::vec2d::to_vec2d;

    #[test]
    fn test_robot_sim_1() {
        let commands = [4, -1, 3].to_vec();
        let obstacles = to_vec2d([[0; 0]; 0]);
        assert_eq!(25, Solution::robot_sim(commands, obstacles));
    }

    #[test]
    fn test_robot_sim_2() {
        let commands = [4, -1, 4, -2, 4].to_vec();
        let obstacles = to_vec2d([[2, 4]]);
        assert_eq!(65, Solution::robot_sim(commands, obstacles));
    }

    #[test]
    fn test_robot_sim_3() {
        let commands = [6, -1, -1, 6].to_vec();
        let obstacles = to_vec2d([[0, 0]]);
        assert_eq!(36, Solution::robot_sim(commands, obstacles));
    }
}
