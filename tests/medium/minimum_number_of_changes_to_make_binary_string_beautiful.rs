// 2914. Minimum Number of Changes to Make Binary String Beautiful
// https://leetcode.com/problems/minimum-number-of-changes-to-make-binary-string-beautiful/

struct Solution;

impl Solution {
    // A beautiful string splits into even-length blocks of identical chars, so every
    // block starts at an even index and each pair (2i, 2i+1) must hold equal chars.
    // Count the mismatching pairs; one flip per pair fixes it.
    pub fn min_changes(s: String) -> i32 {
        s.as_bytes()
            .as_chunks::<2>()
            .0
            .iter()
            .filter(|pair| pair[0] != pair[1])
            .count() as i32
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
