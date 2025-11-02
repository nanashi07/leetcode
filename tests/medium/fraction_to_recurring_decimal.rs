// 166. Fraction to Recurring Decimal
// https://leetcode.com/problems/fraction-to-recurring-decimal/

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
        println!("numerator: {}, denominator: {}", numerator, denominator);

        if numerator == 0 {
            return "0".to_string();
        }

        let mut result = String::new();

        // Handle sign
        if (numerator < 0) ^ (denominator < 0) {
            result.push('-');
        }

        // Work with absolute values to avoid overflow issues
        let mut num = (numerator as i64).abs();
        let den = (denominator as i64).abs();

        // Integer part
        result.push_str(&(num / den).to_string());
        num %= den;

        // If no remainder, return integer result
        if num == 0 {
            return result;
        }

        // Decimal part
        result.push('.');

        // Map to store remainder -> position in decimal part
        let mut remainder_map = HashMap::new();
        let mut decimal_part = String::new();

        while num != 0 {
            // If we've seen this remainder before, we found the recurring part
            if let Some(&pos) = remainder_map.get(&num) {
                // Insert parentheses around the recurring part
                decimal_part.insert(pos, '(');
                decimal_part.push(')');
                break;
            }

            // Record the position of this remainder
            remainder_map.insert(num, decimal_part.len());

            // Perform long division
            num *= 10;
            decimal_part.push_str(&(num / den).to_string());
            num %= den;
        }

        result.push_str(&decimal_part);
        result
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
