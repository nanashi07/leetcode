// 1784. Check if Binary String Has at Most One Segment of Ones
// https://leetcode.com/problems/check-if-binary-string-has-at-most-one-segment-of-ones/

struct Solution;

impl Solution {
    pub fn check_ones_segment(s: String) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::check_if_binary_string_has_at_most_one_segment_of_ones::Solution;

    #[test]
    fn test_check_ones_segment_1() {
        let s = "1001".to_string();
        assert_eq!(false, Solution::check_ones_segment(s));
    }

    #[test]
    fn test_check_ones_segment_2() {
        let s = "110".to_string();
        assert_eq!(true, Solution::check_ones_segment(s));
    }
}
