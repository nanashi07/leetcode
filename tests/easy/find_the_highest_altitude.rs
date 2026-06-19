// 1732. Find the Highest Altitude
// https://leetcode.com/problems/find-the-highest-altitude/

struct Solution;

impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::find_the_highest_altitude::Solution;

    #[test]
    fn test_largest_altitude_1() {
        let gain = [-5, 1, 5, 0, -7].to_vec();
        assert_eq!(1, Solution::largest_altitude(gain));
    }

    #[test]
    fn test_largest_altitude_2() {
        let gain = [-4, -3, -2, -1, 4, 3, 2].to_vec();
        assert_eq!(0, Solution::largest_altitude(gain));
    }
}
