// 3606. Coupon Code Validator
// https://leetcode.com/problems/coupon-code-validator/

struct Solution;

impl Solution {
    pub fn validate_coupons(
        code: Vec<String>,
        business_line: Vec<String>,
        is_active: Vec<bool>,
    ) -> Vec<String> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::coupon_code_validator::Solution;

    #[test]
    fn test_validate_coupons_1() {
        let code = ["SAVE20", "", "PHARMA5", "SAVE@20"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        let business_line = ["restaurant", "grocery", "pharmacy", "restaurant"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        let is_active = [true, true, true, true].to_vec();
        let output = ["PHARMA5", "SAVE20"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        assert_eq!(
            output,
            Solution::validate_coupons(code, business_line, is_active)
        );
    }

    #[test]
    fn test_validate_coupons_2() {
        let code = ["GROCERY15", "ELECTRONICS_50", "DISCOUNT10"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        let business_line = ["grocery", "electronics", "invalid"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        let is_active = [false, true, true].to_vec();
        let output = ["ELECTRONICS_50"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        assert_eq!(
            output,
            Solution::validate_coupons(code, business_line, is_active)
        );
    }
}
