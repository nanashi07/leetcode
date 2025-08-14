// # 2264. Largest 3-Same-Digit Number in String
// https://leetcode.com/problems/largest-3-same-digit-number-in-string/description/?envType=daily-question&envId=2025-08-14

struct Solution;

impl Solution {
    pub fn largest_good_integer(num: String) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::largest_3_same_digit_number_in_string::Solution;

    #[test]
    fn test_largest_good_integer_1() {
        let num = "6777133339".to_owned();
        assert_eq!("777".to_owned(), Solution::largest_good_integer(num));
    }

    #[test]
    fn test_largest_good_integer_2() {
        let num = "2300019".to_owned();
        assert_eq!("000".to_owned(), Solution::largest_good_integer(num));
    }

    #[test]
    fn test_largest_good_integer_3() {
        let num = "42352338".to_owned();
        assert_eq!("".to_owned(), Solution::largest_good_integer(num));
    }
}
