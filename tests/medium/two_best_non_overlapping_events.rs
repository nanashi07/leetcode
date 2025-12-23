// 2054. Two Best Non-Overlapping Events
// https://leetcode.com/problems/two-best-non-overlapping-events/

struct Solution;

impl Solution {
    pub fn max_two_events(events: Vec<Vec<i32>>) -> i32 {
        todo!()
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
