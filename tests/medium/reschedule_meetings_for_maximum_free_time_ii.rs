// # 3440. Reschedule Meetings for Maximum Free Time II
// https://leetcode.com/problems/reschedule-meetings-for-maximum-free-time-ii/

struct Solution;

impl Solution {
    pub fn max_free_time(event_time: i32, start_time: Vec<i32>, end_time: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::reschedule_meetings_for_maximum_free_time_ii::Solution;

    #[test]
    fn test_max_free_time_1() {
        let event_time = 5;
        let start_time = [1, 3].to_vec();
        let end_time = [2, 5].to_vec();
        assert_eq!(2, Solution::max_free_time(event_time, start_time, end_time));
    }

    #[test]
    fn test_max_free_time_2() {
        let event_time = 10;
        let start_time = [0, 7, 9].to_vec();
        let end_time = [1, 8, 10].to_vec();
        assert_eq!(7, Solution::max_free_time(event_time, start_time, end_time));
    }

    #[test]
    fn test_max_free_time_3() {
        let event_time = 10;
        let start_time = [0, 3, 7, 9].to_vec();
        let end_time = [1, 4, 8, 10].to_vec();
        assert_eq!(6, Solution::max_free_time(event_time, start_time, end_time));
    }

    #[test]
    fn test_max_free_time_4() {
        let event_time = 5;
        let start_time = [0, 1, 2, 3, 4].to_vec();
        let end_time = [1, 2, 3, 4, 5].to_vec();
        assert_eq!(0, Solution::max_free_time(event_time, start_time, end_time));
    }
}
