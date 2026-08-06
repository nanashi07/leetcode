// 3345. Smallest Divisible Digit Product I
// https://leetcode.com/problems/smallest-divisible-digit-product-i/

struct Solution;

impl Solution {
    pub fn smallest_number(n: i32, t: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::smallest_divisible_digit_product_i::Solution;

    #[test]
    fn test_smallest_number_1() {
        let n = 10;
        let t = 2;
        assert_eq!(10, Solution::smallest_number(n, t));
    }

    #[test]
    fn test_smallest_number_2() {
        let n = 15;
        let t = 3;
        assert_eq!(16, Solution::smallest_number(n, t));
    }
}
