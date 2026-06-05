// 3753. Total Waviness of Numbers in Range II
// https://leetcode.com/problems/total-waviness-of-numbers-in-range-ii/

struct Solution;

impl Solution {
    pub fn total_waviness(num1: i64, num2: i64) -> i64 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::total_waviness_of_numbers_in_range_ii::Solution;

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
