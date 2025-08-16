// # 1323. Maximum 69 Number
// https://leetcode.com/problems/maximum-69-number/description/?envType=daily-question&envId=2025-08-16

struct Solution;

impl Solution {
    pub fn maximum69_number(num: i32) -> i32 {
        println!("num: {}", num);

        let mut num = num.to_string();
        num.replacen("6", "9", 1).parse().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::maximum_69_number::Solution;

    #[test]
    fn test_maximum69_number_1() {
        let num = 9669;
        assert_eq!(9969, Solution::maximum69_number(num));
    }

    #[test]
    fn test_maximum69_number_2() {
        let num = 9996;
        assert_eq!(9999, Solution::maximum69_number(num));
    }

    #[test]
    fn test_maximum69_number_3() {
        let num = 9999;
        assert_eq!(9999, Solution::maximum69_number(num));
    }
}
