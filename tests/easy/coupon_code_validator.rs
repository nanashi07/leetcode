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
    use crate::shared::vec2d::to_string_vec;

    #[test]
    fn test_validate_coupons_1() {
        let code = to_string_vec(["SAVE20", "", "PHARMA5", "SAVE@20"]);
        let business_line = to_string_vec(["restaurant", "grocery", "pharmacy", "restaurant"]);
        let is_active = [true, true, true, true].to_vec();
        let output = to_string_vec(["PHARMA5", "SAVE20"]);
        assert_eq!(
            output,
            Solution::validate_coupons(code, business_line, is_active)
        );
    }

    #[test]
    fn test_validate_coupons_2() {
        let code = to_string_vec(["GROCERY15", "ELECTRONICS_50", "DISCOUNT10"]);
        let business_line = to_string_vec(["grocery", "electronics", "invalid"]);
        let is_active = [false, true, true].to_vec();
        let output = to_string_vec(["ELECTRONICS_50"]);
        assert_eq!(
            output,
            Solution::validate_coupons(code, business_line, is_active)
        );
    }
}
