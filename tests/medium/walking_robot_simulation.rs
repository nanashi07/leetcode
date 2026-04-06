// 874. Walking Robot Simulation
// https://leetcode.com/problems/walking-robot-simulation/

use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
        let obstacle_set: HashSet<(i32, i32)> = obstacles
            .iter()
            .map(|o| (o[0], o[1]))
            .collect();

        // Directions: N, E, S, W
        let dirs: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut dir = 0usize;
        let (mut x, mut y) = (0i32, 0i32);
        let mut max_dist = 0i32;

        for cmd in commands {
            match cmd {
                -2 => dir = (dir + 3) % 4, // turn left
                -1 => dir = (dir + 1) % 4, // turn right
                steps => {
                    let (dx, dy) = dirs[dir];
                    for _ in 0..steps {
                        let nx = x + dx;
                        let ny = y + dy;
                        if !obstacle_set.contains(&(nx, ny)) {
                            x = nx;
                            y = ny;
                            max_dist = max_dist.max(x * x + y * y);
                        } else {
                            break;
                        }
                    }
                }
            }
        }

        max_dist
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
