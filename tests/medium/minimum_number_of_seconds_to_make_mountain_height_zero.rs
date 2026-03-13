// 3296. Minimum Number of Seconds to Make Mountain Height Zero
// https://leetcode.com/problems/minimum-number-of-seconds-to-make-mountain-height-zero/

struct Solution;

impl Solution {
    pub fn min_number_of_seconds(mountain_height: i32, worker_times: Vec<i32>) -> i64 {
        todo!()
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
