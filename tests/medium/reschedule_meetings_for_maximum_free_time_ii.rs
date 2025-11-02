// 3440. Reschedule Meetings for Maximum Free Time II
// https://leetcode.com/problems/reschedule-meetings-for-maximum-free-time-ii/

use std::cmp::max;

struct Solution;

impl Solution {
    // https://leetcode.com/problems/reschedule-meetings-for-maximum-free-time-ii/solutions/6941677/beats-super-easy-beginners-java-c-c-python-javascript-dart/
    pub fn max_free_time(event_time: i32, start_time: Vec<i32>, end_time: Vec<i32>) -> i32 {
        let n = start_time.len();
        if n == 0 {
            return event_time;
        }

        let mut gaps = vec![0; n + 1];
        gaps[0] = start_time[0];
        for i in 1..n {
            gaps[i] = start_time[i] - end_time[i - 1];
        }
        gaps[n] = event_time - end_time[n - 1];

        let mut largest_right = vec![0; n + 1];
        for i in (0..n).rev() {
            largest_right[i] = max(largest_right[i + 1], gaps[i + 1]);
        }

        let mut max_free = 0;
        let mut largest_left = 0;

        for i in 1..=n {
            let duration = end_time[i - 1] - start_time[i - 1];
            let can_fit_left = largest_left >= duration;
            let can_fit_right = largest_right[i] >= duration;

            if can_fit_left || can_fit_right {
                let merged = gaps[i - 1] + gaps[i] + duration;
                max_free = max(max_free, merged);
            }

            max_free = max(max_free, gaps[i - 1] + gaps[i]);
            largest_left = max(largest_left, gaps[i - 1]);
        }

        max_free
    }

    // failed
    pub fn _max_free_time(event_time: i32, start_time: Vec<i32>, end_time: Vec<i32>) -> i32 {
        println!(
            "event_time: {}, start_time: {:?}, end_time: {:?}",
            event_time, &start_time, &end_time
        );
        let len = start_time.len();
        let mut spaces: Vec<i32> = Vec::new();

        spaces.push(start_time[0]);
        for i in 0..(len - 1) {
            // spaces.push(end_time[i] - start_time[i]);
            spaces.push(start_time[i + 1] - end_time[i]);
        }
        spaces.push(event_time - end_time[end_time.len() - 1]);
        println!("spaces: {:?}", &spaces);

        let mut max = 0;

        for i in 0..spaces.len() {
            // space
            let previous_space = if i == 0 { 0 } else { spaces[i - 1] };
            let previous_meeting = if i == 0 {
                0
            } else {
                end_time[i - 1] - start_time[i - 1]
            };
            let current_space = spaces[i];

            if i > 0 {
                for j in 0..i - 1 {
                    if spaces[j] >= previous_meeting {
                        let sum = previous_space + previous_meeting + current_space;
                        max = max.max(sum);
                        break;
                    }
                }
            }
            for j in i + 1..spaces.len() {
                if spaces[j] >= previous_meeting {
                    let sum = previous_space + previous_meeting + current_space;
                    max = max.max(sum);
                    break;
                }
            }
            max = max.max(previous_space + current_space);
        }

        max
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

    #[test]
    fn test_max_free_time_5() {
        let event_time = 34;
        let start_time = [0, 17].to_vec();
        let end_time = [14, 19].to_vec();
        assert_eq!(
            18,
            Solution::max_free_time(event_time, start_time, end_time)
        );
    }
}
