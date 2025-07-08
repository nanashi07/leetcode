// # 1353. Maximum Number of Events That Can Be Attended
// https://leetcode.com/problems/maximum-number-of-events-that-can-be-attended/

use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    // https://leetcode.com/problems/maximum-number-of-events-that-can-be-attended/solutions/6914754/maximum-number-of-events-that-can-be-attended/
    pub fn max_events(events: Vec<Vec<i32>>) -> i32 {
        let mut events = events;
        // sort events so can iterator accordingly
        events.sort_by(|a, b| a[0].cmp(&b[0]));
        println!("events: {:?}", &events);
        let max_day = events.iter().map(|e| e[1]).max().unwrap_or(0);
        let mut pq = BinaryHeap::new();
        let mut count = 0;
        let mut i = 0;

        // iterator from the first day to last day
        for day in 1..=max_day {
            // get all started events
            while i < events.len() && events[i][0] <= day {
                println!("day: {}, add event: {:?}", day, &events[i]);
                pq.push(Reverse(events[i][1]));
                i += 1;
            }
            println!("all ongoing events: {:?}", &pq);
            // remove expired events (ended time before than day)
            while let Some(&Reverse(end)) = pq.peek() {
                if end < day {
                    pq.pop();
                } else {
                    break;
                }
            }
            println!("joinable events: {:?}", &pq);
            // join the earliest event
            if let Some(Reverse(_)) = pq.pop() {
                count += 1;
            }
            println!("remained ongoing events: {:?}", &pq);
        }
        count
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
