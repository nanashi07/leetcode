// 3120. Count the Number of Special Characters I
// https://leetcode.com/problems/count-the-number-of-special-characters-i/

struct Solution;

impl Solution {
    pub fn number_of_special_chars(word: String) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::count_the_number_of_special_characters_i::Solution;

    #[test]
    fn test_number_of_special_chars_1() {
        let word = "aaAbcBC".to_string();
        assert_eq!(3, Solution::number_of_special_chars(word));
    }

    #[test]
    fn test_number_of_special_chars_2() {
        let word = "abc".to_string();
        assert_eq!(0, Solution::number_of_special_chars(word));
    }

    #[test]
    fn test_number_of_special_chars_3() {
        let word = "abBCab".to_string();
        assert_eq!(1, Solution::number_of_special_chars(word));
    }
}
