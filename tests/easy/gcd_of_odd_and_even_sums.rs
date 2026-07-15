// 3658. GCD of Odd and Even Sums
// https://leetcode.com/problems/gcd-of-odd-and-even-sums/

struct Solution;

impl Solution {
    pub fn gcd_of_odd_even_sums(n: i32) -> i32 {
        Self::gcd((1 + 2 * n - 1) * n / 2, (2 + 2 * n) * n / 2)
    }

    fn gcd(m: i32, n: i32) -> i32 {
        let s = m.min(n);
        let b = m.max(n);
        let d = b % s;
        if d == 0 {
            s
        } else {
            Self::gcd(s, d)
        }
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
