// 1317. Convert Integer to the Sum of Two No-Zero Integers
// https://leetcode.com/problems/convert-integer-to-the-sum-of-two-no-zero-integers/

struct Solution;

impl Solution {
    pub fn get_no_zero_integers(n: i32) -> Vec<i32> {
        println!("n: {n}");

        let mut n = n;
        let mut p = 0;
        let mut r = vec![0; 2];

        while n > 0 {
            let mut v = n % 10;
            if v < 2 && n >= 10 {
                // borrow from next digit
                v += 10;
                n -= 10;
            }

            // split
            r[0] += (v / 2) * 10_i32.pow(p);
            r[1] += (v - v / 2) * 10_i32.pow(p);

            n /= 10;
            p += 1;
        }

        println!("r: {:?}", &r);

        r
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::convert_integer_to_the_sum_of_two_no_zero_integers::Solution;

    #[test]
    fn test_get_no_zero_integers_1() {
        let n = 2;
        assert_eq!(n, Solution::get_no_zero_integers(n).iter().sum());
    }

    #[test]
    fn test_get_no_zero_integers_2() {
        let n = 11;
        assert_eq!(n, Solution::get_no_zero_integers(n).iter().sum());
    }

    #[test]
    fn test_get_no_zero_integers_3() {
        let n = 39;
        assert_eq!(n, Solution::get_no_zero_integers(n).iter().sum());
    }
}
