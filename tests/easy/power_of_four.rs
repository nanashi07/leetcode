// # 342. Power of Four
// https://leetcode.com/problems/power-of-four/

struct Solution;

impl Solution {
    pub fn is_power_of_four(n: i32) -> bool {
        println!("n: {}", n);

        let mut n = n;

        while n > 1 {
            if n % 4 > 0 {
                return false;
            }
            n = n >> 2
        }

        n == 1
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::power_of_four::Solution;

    #[test]
    fn test_is_power_of_four_1() {
        let n = 16;
        assert_eq!(true, Solution::is_power_of_four(n));
    }

    #[test]
    fn test_is_power_of_four_2() {
        let n = 5;
        assert_eq!(false, Solution::is_power_of_four(n));
    }

    #[test]
    fn test_is_power_of_four_3() {
        let n = 1;
        assert_eq!(true, Solution::is_power_of_four(n));
    }
}
