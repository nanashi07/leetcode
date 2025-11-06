// 1751. Maximum Number of Events That Can Be Attended II
// https://leetcode.com/problems/maximum-number-of-events-that-can-be-attended-ii/

struct Solution;

impl Solution {
    // https://leetcode.com/problems/maximum-number-of-events-that-can-be-attended-ii/solutions/3496792/maximum-number-of-events-that-can-be-attended-ii/
    pub fn max_value(events: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut events = events;
        // Sort events by end time for optimal DP transition
        events.sort_unstable_by_key(|event| event[1]);

        let n = events.len();
        let k = k as usize;

        // dp[i][j] = maximum value using first i events with at most j events attended
        let mut dp = vec![vec![0; k + 1]; n + 1];

        for i in 1..=n {
            let start = events[i - 1][0];
            let _end = events[i - 1][1];
            let value = events[i - 1][2];

            // Find the latest event that ends before current event starts
            let mut prev_idx = i - 1;
            while prev_idx > 0 && events[prev_idx - 1][1] >= start {
                prev_idx -= 1;
            }

            for j in 0..=k {
                // Option 1: Don't take current event
                dp[i][j] = dp[i - 1][j];

                // Option 2: Take current event (if we have capacity)
                if j > 0 {
                    let take_current = value + dp[prev_idx][j - 1];
                    dp[i][j] = dp[i][j].max(take_current);
                }
            }
        }

        dp[n][k]
    }

    // Alternative solution with binary search for better time complexity
    pub fn _max_value_optimized(events: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut events = events;
        events.sort_unstable_by_key(|event| event[1]); // Sort by end time

        let n = events.len();
        let k = k as usize;

        // Binary search to find the rightmost event that ends before target start time
        fn binary_search(events: &Vec<Vec<i32>>, target_start: i32, right: usize) -> usize {
            let mut left = 0;
            let mut right = right;
            let mut result = 0;

            while left < right {
                let mid = (left + right) / 2;
                if events[mid][1] < target_start {
                    result = mid + 1;
                    left = mid + 1;
                } else {
                    right = mid;
                }
            }
            result
        }

        let mut dp = vec![vec![0; k + 1]; n + 1];

        for i in 1..=n {
            let start = events[i - 1][0];
            let value = events[i - 1][2];

            // Find latest non-overlapping event using binary search
            let prev_idx = binary_search(&events, start, i - 1);

            for j in 0..=k {
                // Don't take current event
                dp[i][j] = dp[i - 1][j];

                // Take current event
                if j > 0 {
                    let take_current = value + dp[prev_idx][j - 1];
                    dp[i][j] = dp[i][j].max(take_current);
                }
            }
        }

        dp[n][k]
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::maximum_number_of_events_that_can_be_attended_ii::Solution;

    #[test]
    fn test_max_value_1() {
        let events = [[1, 2, 4], [3, 4, 3], [2, 3, 1]]
            .iter()
            .map(|e| e.to_vec())
            .collect::<Vec<Vec<i32>>>();
        let k = 2;
        assert_eq!(7, Solution::max_value(events, k));
    }

    #[test]
    fn test_max_value_2() {
        let events = [[1, 2, 4], [3, 4, 3], [2, 3, 10]]
            .iter()
            .map(|e| e.to_vec())
            .collect::<Vec<Vec<i32>>>();
        let k = 2;
        assert_eq!(10, Solution::max_value(events, k));
    }

    #[test]
    fn test_max_value_3() {
        let events = [[1, 1, 1], [2, 2, 2], [3, 3, 3], [4, 4, 4]]
            .iter()
            .map(|e| e.to_vec())
            .collect::<Vec<Vec<i32>>>();
        let k = 3;
        assert_eq!(9, Solution::max_value(events, k));
    }
}
