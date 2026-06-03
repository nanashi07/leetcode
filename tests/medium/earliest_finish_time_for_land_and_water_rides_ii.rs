// 3635. Earliest Finish Time for Land and Water Rides II
// https://leetcode.com/problems/earliest-finish-time-for-land-and-water-rides-ii/

struct Solution;

impl Solution {
    pub fn earliest_finish_time(
        land_start_time: Vec<i32>,
        land_duration: Vec<i32>,
        water_start_time: Vec<i32>,
        water_duration: Vec<i32>,
    ) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::earliest_finish_time_for_land_and_water_rides_ii::Solution;

    #[test]
    fn test_earliest_finish_time_1() {
        let land_start_time = [2, 8].to_vec();
        let land_duration = [4, 1].to_vec();
        let water_start_time = [6].to_vec();
        let water_duration = [3].to_vec();
        assert_eq!(
            9,
            Solution::earliest_finish_time(
                land_start_time,
                land_duration,
                water_start_time,
                water_duration
            )
        );
    }

    #[test]
    fn test_earliest_finish_time_2() {
        let land_start_time = [5].to_vec();
        let land_duration = [3].to_vec();
        let water_start_time = [1].to_vec();
        let water_duration = [10].to_vec();
        assert_eq!(
            14,
            Solution::earliest_finish_time(
                land_start_time,
                land_duration,
                water_start_time,
                water_duration
            )
        );
    }
}
