// 796. Rotate String
// https://leetcode.com/problems/rotate-string/

struct Solution;

impl Solution {
    pub fn rotate_string(s: String, goal: String) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::rotate_string::Solution;

    #[test]
    fn test_rotate_string_1() {
        let s = "abcde".to_string();
        let goal = "cdeab".to_string();
        assert_eq!(true, Solution::rotate_string(s, goal));
    }

    #[test]
    fn test_rotate_string_2() {
        let s = "abcde".to_string();
        let goal = "abced".to_string();
        assert_eq!(false, Solution::rotate_string(s, goal));
    }
}
