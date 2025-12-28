// 3606. Coupon Code Validator
// https://leetcode.com/problems/coupon-code-validator/

struct Solution;

impl Solution {
    pub fn validate_coupons(
        code: Vec<String>,
        business_line: Vec<String>,
        is_active: Vec<bool>,
    ) -> Vec<String> {
        println!(
            "code: {:?}, business_line: {:?}, is_active: {:?}",
            &code, &business_line, &is_active
        );

        let valid_category = ["electronics", "grocery", "pharmacy", "restaurant"];
        let mut valid_code = vec![];

        for i in 0..is_active.len() {
            if !is_active[i] {
                continue;
            }
            if !valid_category.contains(&business_line[i].as_str()) {
                continue;
            }
            let code = &code[i];
            if !code.is_empty()
                && code
                    .chars()
                    .filter(|c| !c.is_alphanumeric() && c != &'_')
                    .count()
                    == 0
            {
                valid_code.push((
                    code,
                    valid_category
                        .binary_search(&business_line[i].as_str())
                        .unwrap(),
                ));
            }
        }

        valid_code.sort_by(|(c1, i1), (c2, i2)| i1.cmp(i2).then_with(|| c1.cmp(c2)));
        valid_code
            .iter()
            .map(|(c, _)| c.to_string())
            .collect::<Vec<_>>()
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
