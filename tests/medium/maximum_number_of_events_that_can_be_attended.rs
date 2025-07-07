// # 1353. Maximum Number of Events That Can Be Attended
// https://leetcode.com/problems/maximum-number-of-events-that-can-be-attended/

use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    // https://leetcode.com/problems/maximum-number-of-events-that-can-be-attended/solutions/6914754/maximum-number-of-events-that-can-be-attended/
    pub fn max_events(events: Vec<Vec<i32>>) -> i32 {
        let mut events = events;
        events.sort_by(|a, b| a[0].cmp(&b[0]));
        println!("events: {:?}", &events);
        let max_day = events.iter().map(|e| e[1]).max().unwrap_or(0);
        let mut pq = BinaryHeap::new();
        let mut ans = 0;
        let mut j = 0;
        for day in 1..=max_day {
            while j < events.len() && events[j][0] <= day {
                pq.push(Reverse(events[j][1]));
                j += 1;
            }
            println!("j: {}, pg: {:?}", j, &pq);
            while let Some(&Reverse(end)) = pq.peek() {
                if end < day {
                    pq.pop();
                } else {
                    break;
                }
            }
            println!("pg1: {:?}", &pq);
            if let Some(Reverse(_)) = pq.pop() {
                ans += 1;
            }
            println!("day: {}, pg2: {:?}", day, &pq);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::maximum_number_of_events_that_can_be_attended::Solution;

    #[test]
    fn test_max_events_1() {
        let events = [[1, 2], [2, 3], [3, 4]]
            .iter()
            .map(|&e| e.to_vec())
            .collect::<Vec<Vec<i32>>>();
        assert_eq!(3, Solution::max_events(events));
    }

    #[test]
    fn test_max_events_2() {
        let events = [[1, 2], [2, 3], [3, 4], [1, 2]]
            .iter()
            .map(|&e| e.to_vec())
            .collect::<Vec<Vec<i32>>>();
        assert_eq!(4, Solution::max_events(events));
    }

    #[test]
    fn test_max_events_3() {
        let events = [[1, 2], [1, 2], [3, 3], [1, 5], [1, 5]]
            .iter()
            .map(|&e| e.to_vec())
            .collect::<Vec<Vec<i32>>>();
        assert_eq!(5, Solution::max_events(events));
    }

    #[test]
    fn test_max_events_4() {
        let events = [[1, 5], [1, 5], [1, 5], [2, 3], [2, 3]]
            .iter()
            .map(|&e| e.to_vec())
            .collect::<Vec<Vec<i32>>>();
        assert_eq!(5, Solution::max_events(events));
    }

    #[test]
    fn test_max_events_5() {
        let events = [[1, 4], [4, 4], [2, 2], [3, 4], [1, 1]]
            .iter()
            .map(|&e| e.to_vec())
            .collect::<Vec<Vec<i32>>>();
        assert_eq!(4, Solution::max_events(events));
    }
}
