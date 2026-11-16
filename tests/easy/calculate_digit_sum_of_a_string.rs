// 2243. Calculate Digit Sum of a String
// https://leetcode.com/problems/calculate-digit-sum-of-a-string/

struct Solution;

impl Solution {
    pub fn digit_sum(s: String, k: i32) -> String {
        let k = k.max(1) as usize;
        let mut round = s;
        while round.len() > k {
            let mut next = String::with_capacity(round.len());
            for chunk in round.as_bytes().chunks(k) {
                let sum: u32 = chunk.iter().map(|&b| u32::from(b - b'0')).sum();
                next.push_str(&sum.to_string());
            }
            round = next;
        }
        round
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::calculate_digit_sum_of_a_string::Solution;

    #[test]
    fn test_digit_sum_1() {
        let s: String = "11111222223".to_string();
        let k: i32 = 3;
        assert_eq!("135".to_string(), Solution::digit_sum(s, k));
    }

    #[test]
    fn test_digit_sum_2() {
        let s: String = "00000000".to_string();
        let k: i32 = 3;
        assert_eq!("000".to_string(), Solution::digit_sum(s, k));
    }
}
