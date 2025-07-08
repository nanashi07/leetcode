// # 1751. Maximum Number of Events That Can Be Attended II
// https://leetcode.com/problems/maximum-number-of-events-that-can-be-attended-ii/

struct Solution;

impl Solution {
    pub fn max_value(events: Vec<Vec<i32>>, k: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod test {
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
