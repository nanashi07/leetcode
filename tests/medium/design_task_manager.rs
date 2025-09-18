// # 3408. Design Task Manager
// https://leetcode.com/problems/design-task-manager/

use std::cell::RefCell;
use std::cmp::Ordering;
/**
 * Your TaskManager object will be instantiated and called as such:
 * let obj = TaskManager::new(tasks);
 * obj.add(userId, taskId, priority);
 * obj.edit(taskId, newPriority);
 * obj.rmv(taskId);
 * let ret_4: i32 = obj.exec_top();
 */
use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, Clone)]
struct Task {
    user_id: i32,
    task_id: i32,
    priority: i32,
    version: usize, // To handle stale entries in heap
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority && self.task_id == other.task_id
    }
}

impl Eq for Task {}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        // Higher priority first, then higher task_id for tie-breaking
        match self.priority.cmp(&other.priority) {
            Ordering::Equal => self.task_id.cmp(&other.task_id),
            other => other,
        }
    }
}

struct TaskManager {
    tasks: RefCell<HashMap<i32, Task>>,
    heap: RefCell<BinaryHeap<Task>>,
    version_counter: RefCell<usize>,
}
/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TaskManager {
    fn new(tasks: Vec<Vec<i32>>) -> Self {
        let mut task_map = HashMap::new();
        let mut heap = BinaryHeap::new();

        for task_data in tasks {
            let user_id = task_data[0];
            let task_id = task_data[1];
            let priority = task_data[2];

            let task = Task {
                user_id,
                task_id,
                priority,
                version: 0,
            };

            task_map.insert(task_id, task.clone());
            heap.push(task);
        }

        TaskManager {
            tasks: RefCell::new(task_map),
            heap: RefCell::new(heap),
            version_counter: RefCell::new(0),
        }
    }

    fn add(&self, user_id: i32, task_id: i32, priority: i32) {
        let mut version_counter = self.version_counter.borrow_mut();
        *version_counter += 1;
        let version = *version_counter;

        let task = Task {
            user_id,
            task_id,
            priority,
            version,
        };

        self.tasks.borrow_mut().insert(task_id, task.clone());
        self.heap.borrow_mut().push(task);
    }

    fn edit(&self, task_id: i32, new_priority: i32) {
        let mut tasks = self.tasks.borrow_mut();
        if let Some(task) = tasks.get_mut(&task_id) {
            let mut version_counter = self.version_counter.borrow_mut();
            *version_counter += 1;

            task.priority = new_priority;
            task.version = *version_counter;

            // Push updated task to heap
            self.heap.borrow_mut().push(task.clone());
        }
    }

    fn rmv(&self, task_id: i32) {
        self.tasks.borrow_mut().remove(&task_id);
    }

    fn exec_top(&self) -> i32 {
        let mut heap = self.heap.borrow_mut();
        let tasks = self.tasks.borrow();

        // Find the top valid task
        while let Some(top_task) = heap.pop() {
            if let Some(current_task) = tasks.get(&top_task.task_id) {
                // Check if this heap entry is the current version
                if current_task.version == top_task.version {
                    // Remove the task from tasks map
                    drop(tasks);
                    self.tasks.borrow_mut().remove(&top_task.task_id);
                    return top_task.user_id;
                }
                // Otherwise, this is a stale entry, continue to next
            }
            // If task not in map, it was removed, continue to next
        }

        // No tasks available
        -1
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
