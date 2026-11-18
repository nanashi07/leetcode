// 2914. Minimum Number of Changes to Make Binary String Beautiful
// https://leetcode.com/problems/minimum-number-of-changes-to-make-binary-string-beautiful/

struct Solution;

impl Solution {
    pub fn min_changes(s: String) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::minimum_number_of_changes_to_make_binary_string_beautiful::Solution;

    #[test]
    fn test_min_changes_1() {
        let s: String = "1001".to_string();
        assert_eq!(2, Solution::min_changes(s));
    }

    #[test]
    fn test_min_changes_2() {
        let s: String = "10".to_string();
        assert_eq!(1, Solution::min_changes(s));
    }

    #[test]
    fn test_min_changes_3() {
        let s: String = "0000".to_string();
        assert_eq!(0, Solution::min_changes(s));
    }
}
