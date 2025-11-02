// 2749. Minimum Operations to Make the Integer Zero
// https://leetcode.com/problems/minimum-operations-to-make-the-integer-zero/

struct Solution;

impl Solution {
    pub fn make_the_integer_zero(num1: i32, num2: i32) -> i32 {
        println!("num1: {num1}, num2: {num2}");

        let mut num1 = num1 as i64;
        let mut n = 0;

        // v = num1 - (2^a + 2^b + .. + 2^n + n * num2)
        // 2^a + 2^b + .. + 2^n = num1 - n * num2 - v

        // 4 + 4 + 1 = 3 - 3*(-2) = 9

        loop {
            num1 -= num2 as i64;
            n += 1;

            if num2 > 0 && num1 <= 0 {
                return -1;
            } else {
                let b = format!("{:b}", &num1);
                let count = b.chars().filter(|&c| c == '1').count();
                if num1 == 1 {
                    if n == 1 {
                        return n as i32;
                    }
                } else {
                    if (count == 1 && n >= 1) || (count > 1 && count <= n) {
                        return n as i32;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::minimum_operations_to_make_the_integer_zero::Solution;

    #[test]
    fn test_make_the_integer_zero_1() {
        let num1 = 3;
        let num2 = -2;
        assert_eq!(3, Solution::make_the_integer_zero(num1, num2));
    }

    #[test]
    fn test_make_the_integer_zero_2() {
        let num1 = 5;
        let num2 = 7;
        assert_eq!(-1, Solution::make_the_integer_zero(num1, num2));
    }

    #[test]
    fn test_make_the_integer_zero_3() {
        let num1 = 12;
        let num2 = -55;
        assert_eq!(4, Solution::make_the_integer_zero(num1, num2));
    }

    #[test]
    fn test_make_the_integer_zero_4() {
        let num1 = 110;
        let num2 = 55;
        assert_eq!(-1, Solution::make_the_integer_zero(num1, num2));
    }

    #[test]
    fn test_make_the_integer_zero_5() {
        let num1 = 112577768;
        let num2 = -501662198;
        assert_eq!(16, Solution::make_the_integer_zero(num1, num2));
    }

    #[test]
    fn test_make_the_integer_zero_6() {
        let num1 = 85;
        let num2 = 42;
        assert_eq!(-1, Solution::make_the_integer_zero(num1, num2));
    }

    #[test]
    fn test_make_the_integer_zero_7() {
        let num1 = 52;
        let num2 = -12;
        assert_eq!(1, Solution::make_the_integer_zero(num1, num2));
    }

    #[test]
    fn test_make_the_integer_zero_8() {
        let num1 = 34;
        let num2 = 9;
        assert_eq!(2, Solution::make_the_integer_zero(num1, num2));
    }
}
