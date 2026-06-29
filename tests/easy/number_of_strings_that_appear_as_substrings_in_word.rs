// 1967. Number of Strings That Appear as Substrings in Word
// https://leetcode.com/problems/number-of-strings-that-appear-as-substrings-in-word/

struct Solution;

impl Solution {
    pub fn num_of_strings(patterns: Vec<String>, word: String) -> i32 {
        patterns
            .iter()
            .map(|p| if word.contains(p) { 1 } else { 0 })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::number_of_strings_that_appear_as_substrings_in_word::Solution;
    use crate::shared::vec2d::to_string_vec;

    #[test]
    fn test_num_of_strings_1() {
        let patterns = to_string_vec(["a", "abc", "bc", "d"]);
        let word = "abc".to_string();
        assert_eq!(3, Solution::num_of_strings(patterns, word));
    }

    #[test]
    fn test_num_of_strings_2() {
        let patterns = to_string_vec(["a", "b", "c"]);
        let word = "aaaaabbbbb".to_string();
        assert_eq!(2, Solution::num_of_strings(patterns, word));
    }

    #[test]
    fn test_num_of_strings_3() {
        let patterns = to_string_vec(["a", "a", "a"]);
        let word = "ab".to_string();
        assert_eq!(3, Solution::num_of_strings(patterns, word));
    }
}
