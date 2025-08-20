// # 326. Power of Three
// https://leetcode.com/problems/power-of-three/

struct Solution;

impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        let mut n = n;

        if n < 1 {
            false
        } else {
            while n > 1 {
                if n % 3 == 0 {
                    n = n / 3;
                } else {
                    return false;
                }
            }
            n == 1
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::power_of_three::Solution;

    #[test]
    fn test_is_power_of_three_1() {
        let n = 27;
        assert_eq!(true, Solution::is_power_of_three(n));
    }

    #[test]
    fn test_is_power_of_three_2() {
        let n = 0;
        assert_eq!(false, Solution::is_power_of_three(n));
    }

    #[test]
    fn test_is_power_of_three_3() {
        let n = -1;
        assert_eq!(false, Solution::is_power_of_three(n));
    }
}
