// 29. Divide Two Integers
// https://leetcode.com/problems/divide-two-integers/
struct Solution;

impl Solution {
    // ref: https://leetcode.com/problems/divide-two-integers/discuss/13407/C%2B%2B-bit-manipulations
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        // overflow check, special case
        if dividend == i32::MIN && divisor == -1 {
            return i32::MAX;
        }
        // XOR: sone is negative, another is positive
        let sign = (dividend > 0) ^ (divisor > 0);
        // Convert to i64 or else abs(-2147483648) overflows
        let mut dividend = (dividend as i64).abs();
        let divisor = (divisor as i64).abs();
        let mut count: i64 = 0;

        while dividend >= divisor {
            let mut div = divisor;
            let mut times: i64 = 1;

            // find max subtractable
            while dividend > div << 1 && div << 1 > div {
                div = div << 1;
                times = times << 1;
            }

            dividend = dividend - div;
            count = count + times;
        }

        if sign {
            count = count * -1;
        };

        count as i32
    }
}

#[test]
fn test_divide() {
    let dividend = 10;
    let divisor = 3;
    let result = Solution::divide(dividend, divisor);
    assert_eq!(3, result, "dividend: {}, divisor: {}", dividend, divisor);

    let dividend = 7;
    let divisor = -3;
    let result = Solution::divide(dividend, divisor);
    assert_eq!(-2, result, "dividend: {}, divisor: {}", dividend, divisor);

    let dividend = 1;
    let divisor = 1;
    let result = Solution::divide(dividend, divisor);
    assert_eq!(1, result, "dividend: {}, divisor: {}", dividend, divisor);

    let dividend = -1;
    let divisor = 1;
    let result = Solution::divide(dividend, divisor);
    assert_eq!(-1, result, "dividend: {}, divisor: {}", dividend, divisor);

    let dividend = -1;
    let divisor = -1;
    let result = Solution::divide(dividend, divisor);
    assert_eq!(1, result, "dividend: {}, divisor: {}", dividend, divisor);

    let dividend = -2147483648;
    let divisor = -1;
    let result = Solution::divide(dividend, divisor);
    assert_eq!(
        2147483647, result,
        "dividend: {}, divisor: {}",
        dividend, divisor
    );

    let dividend = 2147483647;
    let divisor = 1;
    let result = Solution::divide(dividend, divisor);
    assert_eq!(
        2147483647, result,
        "dividend: {}, divisor: {}",
        dividend, divisor
    );

    let dividend = -2147483648;
    let divisor = 1;
    let result = Solution::divide(dividend, divisor);
    assert_eq!(
        -2147483648, result,
        "dividend: {}, divisor: {}",
        dividend, divisor
    );

    let dividend = -1010369383;
    let divisor = -2147483648;
    let result = Solution::divide(dividend, divisor);
    assert_eq!(0, result, "dividend: {}, divisor: {}", dividend, divisor);
}
