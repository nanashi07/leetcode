// 1009. Complement of Base 10 Integer
// https://leetcode.com/problems/complement-of-base-10-integer/

struct Solution;

impl Solution {
    pub fn bitwise_complement(n: i32) -> i32 {
        println!("n: {n}");

        let mut n = n;
        let mut r = 0;
        let mut i = 0;

        while i == 0 || n > 0 {
            let c = (n % 2 - 1).abs();
            r = r + (c << i);
            n = n / 2;
            i += 1;
        }

        r
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::complement_of_base_10_integer::Solution;

    #[test]
    fn test_bitwise_complement_1() {
        let n = 5;
        assert_eq!(2, Solution::bitwise_complement(n));
    }

    #[test]
    fn test_bitwise_complement_2() {
        let n = 7;
        assert_eq!(0, Solution::bitwise_complement(n));
    }

    #[test]
    fn test_bitwise_complement_3() {
        let n = 10;
        assert_eq!(5, Solution::bitwise_complement(n));
    }

    #[test]
    fn test_bitwise_complement_4() {
        let n = 0;
        assert_eq!(1, Solution::bitwise_complement(n));
    }
}
