// 1882. Process Tasks Using Servers
// https://leetcode.com/problems/process-tasks-using-servers/

struct Solution;

impl Solution {
    pub fn assign_tasks(servers: Vec<i32>, tasks: Vec<i32>) -> Vec<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::process_tasks_using_servers::Solution;

    #[test]
    fn test_assign_tasks_1() {
        let servers = [3, 3, 2].to_vec();
        let tasks = [1, 2, 3, 2, 1, 2].to_vec();
        let output = [2, 2, 0, 2, 1, 2].to_vec();
        assert_eq!(output, Solution::assign_tasks(servers, tasks));
    }

    #[test]
    fn test_assign_tasks_2() {
        let servers = [5, 1, 4, 3, 2].to_vec();
        let tasks = [2, 1, 2, 4, 5, 2, 1].to_vec();
        let output = [1, 4, 1, 4, 1, 3, 2].to_vec();
        assert_eq!(output, Solution::assign_tasks(servers, tasks));
    }
}
