// 3751. Total Waviness of Numbers in Range I
// https://leetcode.com/problems/total-waviness-of-numbers-in-range-i/

struct Solution;

impl Solution {
    pub fn total_waviness(num1: i32, num2: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::total_waviness_of_numbers_in_range_i::Solution;

    #[test]
    fn test_total_waviness_1() {
        let num1 = 120;
        let num2 = 130;
        assert_eq!(3, Solution::total_waviness(num1, num2));
    }

    #[test]
    fn test_total_waviness_2() {
        let num1 = 198;
        let num2 = 202;
        assert_eq!(3, Solution::total_waviness(num1, num2));
    }

    #[test]
    fn test_total_waviness_3() {
        let num1 = 4848;
        let num2 = 4848;
        assert_eq!(2, Solution::total_waviness(num1, num2));
    }
}
