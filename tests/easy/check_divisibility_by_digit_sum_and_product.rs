// 3622. Check Divisibility by Digit Sum and Product
// https://leetcode.com/problems/check-divisibility-by-digit-sum-and-product/

struct Solution;

impl Solution {
    pub fn check_divisibility(n: i32) -> bool {
        let v = n
            .to_string()
            .chars()
            .map(|c| c as i32 - '0' as i32)
            .collect::<Vec<i32>>();
        n % (v.iter().product::<i32>() + v.iter().sum::<i32>()) == 0
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::check_divisibility_by_digit_sum_and_product::Solution;

    #[test]
    fn test_check_divisibility_1() {
        let n = 99;
        assert!(Solution::check_divisibility(n));
    }

    #[test]
    fn test_check_divisibility_2() {
        let n = 23;
        assert!(!Solution::check_divisibility(n));
    }
}
