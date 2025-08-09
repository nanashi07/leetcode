// # 231. Power of Two
// https://leetcode.com/problems/power-of-two/description/?envType=daily-question&envId=2025-08-09

struct Solution;

impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::power_of_two::Solution;

    #[test]
    fn test_is_power_of_two_1() {
        let n = 1;
        assert_eq!(true, Solution::is_power_of_two(n));
    }

    #[test]
    fn test_is_power_of_two_2() {
        let n = 16;
        assert_eq!(true, Solution::is_power_of_two(n));
    }

    #[test]
    fn test_is_power_of_two_3() {
        let n = 3;
        assert_eq!(false, Solution::is_power_of_two(n));
    }
}
