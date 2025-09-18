// # 3408. Design Task Manager
// https://leetcode.com/problems/design-task-manager/

/**
 * Your TaskManager object will be instantiated and called as such:
 * let obj = TaskManager::new(tasks);
 * obj.add(userId, taskId, priority);
 * obj.edit(taskId, newPriority);
 * obj.rmv(taskId);
 * let ret_4: i32 = obj.exec_top();
 */
struct TaskManager {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TaskManager {
    fn new(tasks: Vec<Vec<i32>>) -> Self {
        todo!()
    }

    fn add(&self, user_id: i32, task_id: i32, priority: i32) {
        todo!()
    }

    fn edit(&self, task_id: i32, new_priority: i32) {
        todo!()
    }

    fn rmv(&self, task_id: i32) {
        todo!()
    }

    fn exec_top(&self) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::design_task_manager::TaskManager;

    #[test]
    fn test_task_manager_1() {
        let task_manager = TaskManager::new(
            [[1, 101, 10], [2, 102, 20], [3, 103, 15]]
                .into_iter()
                .map(|task| task.to_vec())
                .collect::<Vec<Vec<_>>>(),
        ); // Initializes with three tasks for Users 1, 2, and 3.
        task_manager.add(4, 104, 5); // Adds task 104 with priority 5 for User 4.
        task_manager.edit(102, 8); // Updates priority of task 102 to 8.
        assert_eq!(3, task_manager.exec_top()); // return 3. Executes task 103 for User 3.
        task_manager.rmv(101); // Removes task 101 from the system.
        task_manager.add(5, 105, 15); // Adds task 105 with priority 15 for User 5.
        assert_eq!(5, task_manager.exec_top()); // return 5. Executes task 105 for User 5.}
    }
}
