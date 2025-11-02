// 3370. Smallest Number With All Set Bits
// https://leetcode.com/problems/smallest-number-with-all-set-bits/

struct Solution;

impl Solution {
    pub fn smallest_number(n: i32) -> i32 {
        println!("n: {n}");

        let mut x = 1;

        while x < n {
            x = 1 + (x << 1);
        }

        x
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::smallest_number_with_all_set_bits::Solution;

    #[test]
    fn test_smallest_number_1() {
        let n = 5;
        assert_eq!(7, Solution::smallest_number(n));
    }

    #[test]
    fn test_smallest_number_2() {
        let n = 10;
        assert_eq!(15, Solution::smallest_number(n));
    }

    #[test]
    fn test_smallest_number_3() {
        let n = 3;
        assert_eq!(3, Solution::smallest_number(n));
    }
}
