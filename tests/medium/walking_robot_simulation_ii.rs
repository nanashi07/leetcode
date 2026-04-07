// 2069. Walking Robot Simulation II
// https://leetcode.com/problems/walking-robot-simulation-ii/

struct Robot {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Robot {
    fn new(width: i32, height: i32) -> Self {
        todo!()
    }

    fn step(&self, num: i32) {
        todo!()
    }

    fn get_pos(&self) -> Vec<i32> {
        todo!()
    }

    fn get_dir(&self) -> String {
        todo!()
    }
}

/**
 * Your Robot object will be instantiated and called as such:
 * let obj = Robot::new(width, height);
 * obj.step(num);
 * let ret_2: Vec<i32> = obj.get_pos();
 * let ret_3: String = obj.get_dir();
 */
#[cfg(test)]
mod tests {
    use crate::medium::walking_robot_simulation_ii::Robot;

    #[test]
    fn test_robot_1() {
        // Input:
        // ["Robot", "step", "step", "getPos", "getDir", "step", "step", "step", "getPos", "getDir"]
        // [[6, 3],  [2],    [2],    [],       [],       [2],    [1],    [4],    [],       []]
        // Output:
        // [null,    null,   null,   [4, 0],   "East",   null,   null,   null,   [1, 2],   "West"]
        let robot = Robot::new(6, 3);
        robot.step(2);
        robot.step(2);
        assert_eq!(vec![4, 0], robot.get_pos());
        assert_eq!("East".to_string(), robot.get_dir());
        robot.step(2);
        robot.step(1);
        robot.step(4);
        assert_eq!(vec![1, 2], robot.get_pos());
        assert_eq!("West".to_string(), robot.get_dir());
    }
}
