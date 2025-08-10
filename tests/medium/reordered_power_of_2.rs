// # 869. Reordered Power of 2
// https://leetcode.com/problems/reordered-power-of-2/

struct Solution;

impl Solution {
    pub fn reordered_power_of2(n: i32) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::reordered_power_of_2::Solution;

    #[test]
    fn test_reordered_power_of2_1() {
        let n = 1;
        assert_eq!(true, Solution::reordered_power_of2(n));
    }

    #[test]
    fn test_reordered_power_of2_2() {
        let n = 10;
        assert_eq!(true, Solution::reordered_power_of2(n));
    }
}
