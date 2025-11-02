// 231. Power of Two
// https://leetcode.com/problems/power-of-two/

struct Solution;

impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        if n < 1 {
            return false;
        }

        let mut n = n;

        while n > 1 {
            if n % 2 != 0 {
                return false;
            }
            n = n >> 1;
        }

        true
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
