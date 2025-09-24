// # 166. Fraction to Recurring Decimal
// https://leetcode.com/problems/fraction-to-recurring-decimal/

struct Solution;

impl Solution {
    pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::fraction_to_recurring_decimal::Solution;

    #[test]
    fn test_fraction_to_decimal_1() {
        let numerator = 1;
        let denominator = 2;
        assert_eq!(
            "0.5".to_string(),
            Solution::fraction_to_decimal(numerator, denominator)
        );
    }

    #[test]
    fn test_fraction_to_decimal_2() {
        let numerator = 2;
        let denominator = 1;
        assert_eq!(
            "2".to_string(),
            Solution::fraction_to_decimal(numerator, denominator)
        );
    }

    #[test]
    fn test_fraction_to_decimal_3() {
        let numerator = 4;
        let denominator = 333;
        assert_eq!(
            "0.(012)".to_string(),
            Solution::fraction_to_decimal(numerator, denominator)
        );
    }
}
