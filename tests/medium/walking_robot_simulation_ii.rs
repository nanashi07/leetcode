// 2069. Walking Robot Simulation II
// https://leetcode.com/problems/walking-robot-simulation-ii/

use std::cell::Cell;

struct Robot {
    width: i32,
    height: i32,
    perimeter: i32,
    offset: Cell<i32>,
    moved: Cell<bool>,
}

impl Robot {
    fn new(width: i32, height: i32) -> Self {
        Robot {
            width,
            height,
            perimeter: 2 * (width - 1) + 2 * (height - 1),
            offset: Cell::new(0),
            moved: Cell::new(false),
        }
    }

    fn step(&self, num: i32) {
        self.offset.set((self.offset.get() + num) % self.perimeter);
        self.moved.set(true);
    }

    fn get_pos(&self) -> Vec<i32> {
        let off = self.offset.get();
        let w = self.width;
        let h = self.height;
        if off <= w - 1 {
            vec![off, 0]
        } else if off <= w - 1 + h - 1 {
            vec![w - 1, off - (w - 1)]
        } else if off <= 2 * (w - 1) + h - 1 {
            vec![w - 1 - (off - (w - 1) - (h - 1)), h - 1]
        } else {
            vec![0, h - 1 - (off - 2 * (w - 1) - (h - 1))]
        }
    }

    fn get_dir(&self) -> String {
        if !self.moved.get() {
            return "East".to_string();
        }
        let off = self.offset.get();
        let w = self.width;
        let h = self.height;
        if off > 0 && off <= w - 1 {
            "East"
        } else if off > w - 1 && off <= w - 1 + h - 1 {
            "North"
        } else if off > w - 1 + h - 1 && off <= 2 * (w - 1) + h - 1 {
            "West"
        } else {
            "South"
        }
        .to_string()
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
