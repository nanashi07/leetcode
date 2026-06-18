// 1344. Angle Between Hands of a Clock
// https://leetcode.com/problems/angle-between-hands-of-a-clock/

struct Solution;

impl Solution {
    pub fn angle_clock(hour: i32, minutes: i32) -> f64 {
        let minute_angle = minutes as f64 * 6.0;
        let hour_angle = (hour % 12) as f64 * 30.0 + minutes as f64 * 0.5;
        let diff = (minute_angle - hour_angle).abs();
        diff.min(360.0 - diff)
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::angle_between_hands_of_a_clock::Solution;

    #[test]
    fn test_angle_clock_1() {
        let hour = 12;
        let minutes = 30;
        assert_eq!(165f64, Solution::angle_clock(hour, minutes));
    }

    #[test]
    fn test_angle_clock_2() {
        let hour = 3;
        let minutes = 30;
        assert_eq!(165f64, Solution::angle_clock(hour, minutes));
    }

    #[test]
    fn test_angle_clock_3() {
        let hour = 3;
        let minutes = 15;
        assert_eq!(165f64, Solution::angle_clock(hour, minutes));
    }
}
