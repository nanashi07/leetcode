// 2054. Two Best Non-Overlapping Events
// https://leetcode.com/problems/two-best-non-overlapping-events/

struct Solution;

impl Solution {
    pub fn max_two_events(events: Vec<Vec<i32>>) -> i32 {
        let mut events = events;
        // Sort events by start time
        events.sort_by_key(|e| e[0]);

        let n = events.len();

        // max_value_after[i] stores the maximum value from events[i..n]
        let mut max_value_after = vec![0; n];
        max_value_after[n - 1] = events[n - 1][2];

        for i in (0..n - 1).rev() {
            max_value_after[i] = max_value_after[i + 1].max(events[i][2]);
        }

        let mut result = 0;

        // For each event, consider:
        // 1. Taking only this event
        // 2. Taking this event + the best non-overlapping event after it
        for i in 0..n {
            let current_value = events[i][2];
            result = result.max(current_value);

            // Binary search for the first event that starts after current event ends
            let current_end = events[i][1];
            let mut left = i + 1;
            let mut right = n;

            while left < right {
                let mid = left + (right - left) / 2;
                if events[mid][0] > current_end {
                    right = mid;
                } else {
                    left = mid + 1;
                }
            }

            // If we found a non-overlapping event
            if left < n {
                result = result.max(current_value + max_value_after[left]);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::two_best_non_overlapping_events::Solution;

    #[test]
    fn test_max_two_events_1() {
        let events = [[1, 3, 2], [4, 5, 2], [2, 4, 3]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(4, Solution::max_two_events(events));
    }

    #[test]
    fn test_max_two_events_2() {
        let events = [[1, 3, 2], [4, 5, 2], [1, 5, 5]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(5, Solution::max_two_events(events));
    }

    #[test]
    fn test_max_two_events_3() {
        let events = [[1, 5, 3], [1, 5, 1], [6, 6, 5]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(8, Solution::max_two_events(events));
    }
}
