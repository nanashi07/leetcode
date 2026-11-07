// 1882. Process Tasks Using Servers
// https://leetcode.com/problems/process-tasks-using-servers/

struct Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn assign_tasks(servers: Vec<i32>, tasks: Vec<i32>) -> Vec<i32> {
        // `idle` holds free servers as (weight, index); `busy` holds servers at work
        // as (time they become free, weight, index). Both are min-heaps, so the
        // preferred server per the problem rules is always on top.
        let mut idle: BinaryHeap<Reverse<(i32, usize)>> = BinaryHeap::with_capacity(servers.len());
        for (idx, &weight) in servers.iter().enumerate() {
            idle.push(Reverse((weight, idx)));
        }
        let mut busy: BinaryHeap<Reverse<(i64, i32, usize)>> = BinaryHeap::new();

        let mut answer = Vec::with_capacity(tasks.len());
        for (i, &duration) in tasks.iter().enumerate() {
            let now = i as i64;
            // Release every server whose task finished at or before this second.
            while let Some(&Reverse((free, _, _))) = busy.peek() {
                if free > now {
                    break;
                }
                let Reverse((_, weight, idx)) =
                    busy.pop().expect("peek proved the heap is not empty");
                idle.push(Reverse((weight, idx)));
            }

            let (start, weight, idx) = if let Some(Reverse(entry)) = idle.pop() {
                (now, entry.0, entry.1)
            } else {
                // All servers are busy: wait for the one that frees up earliest.
                let Reverse((free, weight, idx)) =
                    busy.pop().expect("at least one server is always in use");
                (free, weight, idx)
            };

            answer.push(idx as i32);
            busy.push(Reverse((start + duration as i64, weight, idx)));
        }

        answer
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
