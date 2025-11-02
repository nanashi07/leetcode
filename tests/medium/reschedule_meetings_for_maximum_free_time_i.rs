// 3439. Reschedule Meetings for Maximum Free Time I
// https://leetcode.com/problems/reschedule-meetings-for-maximum-free-time-i/

struct Solution;

impl Solution {
    pub fn max_free_time(event_time: i32, k: i32, start_time: Vec<i32>, end_time: Vec<i32>) -> i32 {
        println!(
            "event_time: {}, k: {}, start_time: {:?}, end_time: {:?}",
            event_time, k, &start_time, &end_time
        );
        let len = start_time.len();
        let mut spaces: Vec<i32> = Vec::new();
        spaces.push(start_time[0]);
        for i in 0..len - 1 {
            let end = end_time[i];
            spaces.push(start_time[i + 1] - end);
        }
        spaces.push(event_time - end_time[end_time.len() - 1]);
        println!("spaces: {:?}", &spaces);

        // calculate max sum
        let mut sum: i32 = spaces.iter().take(k as usize + 1).sum();
        let mut max = sum;
        println!("max : {}", sum);
        for i in 1..(spaces.len() - k as usize) {
            sum = sum - spaces[i - 1] + spaces[i + k as usize];
            max = max.max(sum);
        }

        max
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::reschedule_meetings_for_maximum_free_time_i::Solution;

    #[test]
    fn test_max_free_time_1() {
        let event_time = 5;
        let k = 1;
        let start_time = [1, 3].to_vec();
        let end_time = [2, 5].to_vec();
        assert_eq!(
            2,
            Solution::max_free_time(event_time, k, start_time, end_time)
        );
    }

    #[test]
    fn test_max_free_time_2() {
        let event_time = 10;
        let k = 1;
        let start_time = [0, 2, 9].to_vec();
        let end_time = [1, 4, 10].to_vec();
        assert_eq!(
            6,
            Solution::max_free_time(event_time, k, start_time, end_time)
        );
    }

    #[test]
    fn test_max_free_time_3() {
        let event_time = 5;
        let k = 2;
        let start_time = [0, 1, 2, 3, 4].to_vec();
        let end_time = [1, 2, 3, 4, 5].to_vec();
        assert_eq!(
            0,
            Solution::max_free_time(event_time, k, start_time, end_time)
        );
    }

    #[test]
    fn test_max_free_time_4() {
        let event_time = 21;
        let k = 1;
        let start_time = [7, 10, 16].to_vec();
        let end_time = [10, 14, 18].to_vec();
        assert_eq!(
            7,
            Solution::max_free_time(event_time, k, start_time, end_time)
        );
    }
}
