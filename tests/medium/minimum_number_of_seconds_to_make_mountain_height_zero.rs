// 3296. Minimum Number of Seconds to Make Mountain Height Zero
// https://leetcode.com/problems/minimum-number-of-seconds-to-make-mountain-height-zero/

struct Solution;

impl Solution {
    pub fn min_number_of_seconds(mountain_height: i32, worker_times: Vec<i32>) -> i64 {
        let target = mountain_height as i64;
        let fastest_worker = *worker_times.iter().min().unwrap() as i64;
        let mut left = 0_i64;
        let mut right = fastest_worker * target * (target + 1) / 2;

        while left < right {
            let mid = left + (right - left) / 2;
            if Self::can_finish(target, &worker_times, mid) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        left
    }

    fn can_finish(target: i64, worker_times: &[i32], seconds: i64) -> bool {
        let mut removed = 0_i64;

        for &worker_time in worker_times {
            removed += Self::max_height_removed(worker_time as i64, seconds, target - removed);
            if removed >= target {
                return true;
            }
        }

        false
    }

    fn max_height_removed(worker_time: i64, seconds: i64, upper_bound: i64) -> i64 {
        let mut left = 0_i64;
        let mut right = upper_bound;

        while left < right {
            let mid = left + (right - left + 1) / 2;
            let cost = worker_time as i128 * mid as i128 * (mid + 1) as i128 / 2;

            if cost <= seconds as i128 {
                left = mid;
            } else {
                right = mid - 1;
            }
        }

        left
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::minimum_number_of_seconds_to_make_mountain_height_zero::Solution;

    #[test]
    fn test_min_number_of_seconds_1() {
        let mountain_height = 4;
        let worker_times = [2, 1, 1].to_vec();
        assert_eq!(
            3,
            Solution::min_number_of_seconds(mountain_height, worker_times)
        );
    }

    #[test]
    fn test_min_number_of_seconds_2() {
        let mountain_height = 10;
        let worker_times = [3, 2, 2, 4].to_vec();
        assert_eq!(
            12,
            Solution::min_number_of_seconds(mountain_height, worker_times)
        );
    }

    #[test]
    fn test_min_number_of_seconds_3() {
        let mountain_height = 5;
        let worker_times = [1].to_vec();
        assert_eq!(
            15,
            Solution::min_number_of_seconds(mountain_height, worker_times)
        );
    }
}
