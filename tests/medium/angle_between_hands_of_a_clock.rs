// 1344. Angle Between Hands of a Clock
// https://leetcode.com/problems/angle-between-hands-of-a-clock/

struct Solution;

impl Solution {
    pub fn angle_clock(hour: i32, minutes: i32) -> f64 {
        todo!()
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
