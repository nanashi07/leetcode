// 3723. Maximize Sum of Squares of Digits
// https://leetcode.com/problems/maximize-sum-of-squares-of-digits/

struct Solution;

impl Solution {
    pub fn max_sum_of_squares(num: i32, sum: i32) -> String {
        // A num digit number holds a digit sum of at most 9 * num.
        if sum as i64 > num as i64 * 9 {
            return String::new();
        }

        // Squaring is convex, so the maximum score pushes digits to the bounds:
        // as many 9s as possible, one partial digit, then zeros. Emitting them
        // in descending order also gives the largest such integer.
        let nines = (sum / 9) as usize;
        let rest = (sum % 9) as u8;
        let zeros = num as usize - nines - if rest > 0 { 1 } else { 0 };

        let mut ans = String::with_capacity(num as usize);
        ans.push_str(&"9".repeat(nines));
        if rest > 0 {
            ans.push((b'0' + rest) as char);
        }
        ans.push_str(&"0".repeat(zeros));
        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::maximize_sum_of_squares_of_digits::Solution;

    #[test]
    fn test_max_sum_of_squares_1() {
        let num = 2;
        let sum = 3;
        assert_eq!("30".to_string(), Solution::max_sum_of_squares(num, sum));
    }

    #[test]
    fn test_max_sum_of_squares_2() {
        let num = 2;
        let sum = 17;
        assert_eq!("98".to_string(), Solution::max_sum_of_squares(num, sum));
    }

    #[test]
    fn test_max_sum_of_squares_3() {
        let num = 1;
        let sum = 10;
        assert_eq!("".to_string(), Solution::max_sum_of_squares(num, sum));
    }
}
