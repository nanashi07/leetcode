// 3348. Smallest Divisible Digit Product II
// https://leetcode.com/problems/smallest-divisible-digit-product-ii/

struct Solution;

impl Solution {
    pub fn smallest_number(num: String, t: i64) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::smallest_divisible_digit_product_ii::Solution;

    #[test]
    fn test_smallest_number_1() {
        let num = "1234".to_string();
        let t = 256;
        assert_eq!("1488".to_string(), Solution::smallest_number(num, t));
    }

    #[test]
    fn test_smallest_number_2() {
        let num = "12355".to_string();
        let t = 50;
        assert_eq!("12355".to_string(), Solution::smallest_number(num, t));
    }

    #[test]
    fn test_smallest_number_3() {
        let num = "11111".to_string();
        let t = 26;
        assert_eq!("-1".to_string(), Solution::smallest_number(num, t));
    }
}
