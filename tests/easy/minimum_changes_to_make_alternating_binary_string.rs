// 1758. Minimum Changes To Make Alternating Binary String
// https://leetcode.com/problems/minimum-changes-to-make-alternating-binary-string/

struct Solution;

impl Solution {
    pub fn min_operations(s: String) -> i32 {
        let mut zero_start = 0;
        let mut one_start = 0;

        for (i, c) in s.chars().enumerate() {
            if i % 2 == 0 {
                if c == '0' {
                    one_start += 1;
                } else {
                    zero_start += 1;
                }
            } else {
                if c == '0' {
                    zero_start += 1;
                } else {
                    one_start += 1;
                }
            }
        }

        zero_start.min(one_start)
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::minimum_changes_to_make_alternating_binary_string::Solution;

    #[test]
    fn test_min_operations_1() {
        let s = "0100".to_string();
        assert_eq!(1, Solution::min_operations(s));
    }

    #[test]
    fn test_min_operations_2() {
        let s = "10".to_string();
        assert_eq!(0, Solution::min_operations(s));
    }

    #[test]
    fn test_min_operations_3() {
        let s = "1111".to_string();
        assert_eq!(2, Solution::min_operations(s));
    }
}
