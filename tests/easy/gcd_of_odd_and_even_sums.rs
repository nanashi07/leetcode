// 3658. GCD of Odd and Even Sums
// https://leetcode.com/problems/gcd-of-odd-and-even-sums/

struct Solution;

impl Solution {
    pub fn gcd_of_odd_even_sums(n: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::gcd_of_odd_and_even_sums::Solution;

    #[test]
    fn test_gcd_of_odd_even_sums_1() {
        let n = 4;
        assert_eq!(4, Solution::gcd_of_odd_even_sums(n));
    }

    #[test]
    fn test_gcd_of_odd_even_sums_2() {
        let n = 5;
        assert_eq!(5, Solution::gcd_of_odd_even_sums(n));
    }
}
