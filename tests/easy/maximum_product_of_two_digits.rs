// 3536. Maximum Product of Two Digits
// https://leetcode.com/problems/maximum-product-of-two-digits/

struct Solution;

impl Solution {
    pub fn max_product(n: i32) -> i32 {
        if n < 10 {
            n
        } else {
            let mut n = n;
            let mut a = 0;
            let mut b = 0;
            while n > 0 {
                let c = n % 10;
                if a < c {
                    b = a;
                    a = c;
                } else {
                    if b < c {
                        b = c;
                    }
                }
                n = n / 10;
            }
            a * b
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::maximum_product_of_two_digits::Solution;

    #[test]
    fn test_max_product_1() {
        let n = 31;
        assert_eq!(3, Solution::max_product(n));
    }

    #[test]
    fn test_max_product_2() {
        let n = 22;
        assert_eq!(4, Solution::max_product(n));
    }

    #[test]
    fn test_max_product_3() {
        let n = 124;
        assert_eq!(8, Solution::max_product(n));
    }
}
