// # 326. Power of Three
// https://leetcode.com/problems/power-of-three/description/?envType=daily-question&envId=2025-08-13

struct Solution;

impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::power_of_three::Solution;

    #[test]
    fn test_is_power_of_three_1() {
        let n = 27;
        assert!(Solution::is_power_of_three(n));
    }

    #[test]
    fn test_is_power_of_three_2() {
        let n = 0;
        assert!(Solution::is_power_of_three(n));
    }

    #[test]
    fn test_is_power_of_three_3() {
        let n = -1;
        assert!(Solution::is_power_of_three(n));
    }
}
