// 2528. Maximize the Minimum Powered City
// https://leetcode.com/problems/maximize-the-minimum-powered-city/

struct Solution;

impl Solution {
    pub fn max_power(stations: Vec<i32>, r: i32, k: i32) -> i64 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::maximize_the_minimum_powered_city::Solution;

    #[test]
    fn test_max_power_1() {
        let stations = [1, 2, 4, 5, 0].to_vec();
        let r = 1;
        let k = 2;
        assert_eq!(5, Solution::max_power(stations, r, k));
    }

    #[test]
    fn test_max_power_2() {
        let stations = [4, 4, 4, 4].to_vec();
        let r = 0;
        let k = 3;
        assert_eq!(4, Solution::max_power(stations, r, k));
    }
}
