// # 1353. Maximum Number of Events That Can Be Attended
// https://leetcode.com/problems/maximum-number-of-events-that-can-be-attended/

struct Solution;

impl Solution {
    pub fn max_events(events: Vec<Vec<i32>>) -> i32 {
        todo!()
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
}
