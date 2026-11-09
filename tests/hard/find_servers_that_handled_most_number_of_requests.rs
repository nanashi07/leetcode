// 1606. Find Servers That Handled Most Number of Requests
// https://leetcode.com/problems/find-servers-that-handled-most-number-of-requests/

use std::cmp::Reverse;
use std::collections::{BTreeSet, BinaryHeap};

struct Solution;

impl Solution {
    pub fn busiest_servers(k: i32, arrival: Vec<i32>, load: Vec<i32>) -> Vec<i32> {
        let k = k as usize;

        // Busy servers as a min heap keyed by the time they become free again.
        let mut busy: BinaryHeap<Reverse<(i64, usize)>> = BinaryHeap::new();
        // Free server ids kept sorted, so the smallest id >= i % k is a range query.
        let mut free: BTreeSet<usize> = (0..k).collect();
        let mut handled = vec![0; k];

        for (i, (&arrive, &weight)) in arrival.iter().zip(load.iter()).enumerate() {
            // Release every server whose request finished at or before this arrival.
            while busy
                .peek()
                .is_some_and(|&Reverse((end, _))| end <= arrive as i64)
            {
                if let Some(Reverse((_, id))) = busy.pop() {
                    free.insert(id);
                }
            }

            // All servers are still busy: this request is dropped.
            let target = i % k;
            let id = free
                .range(target..)
                .next()
                .or_else(|| free.first())
                .copied();
            if let Some(id) = id {
                free.remove(&id);
                handled[id] += 1;
                busy.push(Reverse((arrive as i64 + weight as i64, id)));
            }
        }

        let mut max_handled = 0;
        for &count in &handled {
            if count > max_handled {
                max_handled = count;
            }
        }

        handled
            .iter()
            .enumerate()
            .filter(|&(_, &count)| count == max_handled)
            .map(|(id, _)| id as i32)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::find_servers_that_handled_most_number_of_requests::Solution;

    #[test]
    fn test_busiest_servers_1() {
        let k = 3;
        let arrival = [1, 2, 3, 4, 5].to_vec();
        let load = [5, 2, 3, 3, 3].to_vec();
        assert_eq!([1].to_vec(), Solution::busiest_servers(k, arrival, load));
    }

    #[test]
    fn test_busiest_servers_2() {
        let k = 3;
        let arrival = [1, 2, 3, 4].to_vec();
        let load = [1, 2, 1, 2].to_vec();
        assert_eq!([0].to_vec(), Solution::busiest_servers(k, arrival, load));
    }

    #[test]
    fn test_busiest_servers_3() {
        let k = 3;
        let arrival = [1, 2, 3].to_vec();
        let load = [10, 12, 11].to_vec();
        assert_eq!(
            [0, 1, 2].to_vec(),
            Solution::busiest_servers(k, arrival, load)
        );
    }
}
