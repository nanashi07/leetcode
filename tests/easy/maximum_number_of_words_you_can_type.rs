// # 1935. Maximum Number of Words You Can Type
// https://leetcode.com/problems/maximum-number-of-words-you-can-type/

struct Solution;

impl Solution {
    pub fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::maximum_number_of_words_you_can_type::Solution;

    #[test]
    fn test_can_be_typed_words_1() {
        let text = "hello world".to_string();
        let broken_letters = "ad".to_string();
        assert_eq!(1, Solution::can_be_typed_words(text, broken_letters));
    }

    #[test]
    fn test_can_be_typed_words_2() {
        let text = "leet code".to_string();
        let broken_letters = "lt".to_string();
        assert_eq!(1, Solution::can_be_typed_words(text, broken_letters));
    }

    #[test]
    fn test_can_be_typed_words_3() {
        let text = "leet code".to_string();
        let broken_letters = "e".to_string();
        assert_eq!(0, Solution::can_be_typed_words(text, broken_letters));
    }
}
