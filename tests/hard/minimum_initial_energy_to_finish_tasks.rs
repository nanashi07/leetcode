// 1665. Minimum Initial Energy to Finish Tasks
// https://leetcode.com/problems/minimum-initial-energy-to-finish-tasks/

struct Solution;

impl Solution {
    pub fn minimum_effort(tasks: Vec<Vec<i32>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::minimum_initial_energy_to_finish_tasks::Solution;
    use crate::shared::vec2d::to_vec2d;

    #[test]
    fn test_minimum_effort_1() {
        let tasks = to_vec2d([[1, 2], [2, 4], [4, 8]]);
        assert_eq!(8, Solution::minimum_effort(tasks));
    }

    #[test]
    fn test_minimum_effort_2() {
        let tasks = to_vec2d([[1, 3], [2, 4], [10, 11], [10, 12], [8, 9]]);
        assert_eq!(32, Solution::minimum_effort(tasks));
    }

    #[test]
    fn test_minimum_effort_3() {
        let tasks = to_vec2d([[1, 7], [2, 8], [3, 9], [4, 10], [5, 11], [6, 12]]);
        assert_eq!(27, Solution::minimum_effort(tasks));
    }
}
