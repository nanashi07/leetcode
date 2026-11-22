// 2124. Check if All A's Appears Before All B's
// https://leetcode.com/problems/check-if-all-as-appears-before-all-bs/

struct Solution;

impl Solution {
    pub fn check_string(s: String) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::check_if_all_as_appears_before_all_bs::Solution;

    #[test]
    fn test_check_string_1() {
        let s: String = "aaabbb".to_string();
        assert!(Solution::check_string(s));
    }

    #[test]
    fn test_check_string_2() {
        let s: String = "abab".to_string();
        assert!(!Solution::check_string(s));
    }

    #[test]
    fn test_check_string_3() {
        let s: String = "bbb".to_string();
        assert!(Solution::check_string(s));
    }
}
