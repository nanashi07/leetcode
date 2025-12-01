// 2141. Maximum Running Time of N Computers
// https://leetcode.com/problems/maximum-running-time-of-n-computers/

struct Solution;

impl Solution {
    pub fn max_run_time(n: i32, batteries: Vec<i32>) -> i64 {
        let n = n as i64;

        // Binary search on the answer
        let mut left = 1i64;
        let mut right = batteries.iter().map(|&x| x as i64).sum::<i64>() / n;

        while left < right {
            let mid = left + (right - left + 1) / 2;

            if Self::can_run(n, &batteries, mid) {
                left = mid;
            } else {
                right = mid - 1;
            }
        }

        left
    }

    fn can_run(n: i64, batteries: &Vec<i32>, time: i64) -> bool {
        // Calculate total available energy, capping each battery at 'time'
        // If a battery has more than 'time', it can only serve one computer
        // for the entire duration, so we cap its contribution at 'time'
        let total_energy: i64 = batteries.iter().map(|&b| (b as i64).min(time)).sum();

        // We need n * time total energy to run n computers for 'time' minutes
        total_energy >= n * time
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::maximum_running_time_of_n_computers::Solution;

    #[test]
    fn test_max_run_time_1() {
        let n = 2;
        let batteries = [3, 3, 3].to_vec();
        assert_eq!(4, Solution::max_run_time(n, batteries));
    }

    #[test]
    fn test_max_run_time_2() {
        let n = 2;
        let batteries = [1, 1, 1, 1].to_vec();
        assert_eq!(2, Solution::max_run_time(n, batteries));
    }
}
