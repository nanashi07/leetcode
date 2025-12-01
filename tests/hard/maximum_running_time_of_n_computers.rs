// 2141. Maximum Running Time of N Computers
// https://leetcode.com/problems/maximum-running-time-of-n-computers/

struct Solution;

impl Solution {
    pub fn max_run_time(n: i32, batteries: Vec<i32>) -> i64 {
        todo!()
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
