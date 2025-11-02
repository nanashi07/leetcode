// 1935. Maximum Number of Words You Can Type
// https://leetcode.com/problems/maximum-number-of-words-you-can-type/

use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
        println!("text: {:?}, broken_letters: {:?}", &text, &broken_letters);

        let letters = broken_letters.chars().into_iter().collect::<HashSet<_>>();
        text.split(' ')
            .filter(|&w| w.chars().filter(|c| letters.contains(c)).count() == 0)
            .count() as i32
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
