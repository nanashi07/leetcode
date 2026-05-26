// 3120. Count the Number of Special Characters I
// https://leetcode.com/problems/count-the-number-of-special-characters-i/

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn number_of_special_chars(word: String) -> i32 {
        let upper_end = 'Z' as i32;
        let diff = 'a' as i32 - 'A' as i32;
        let mut lowers = word
            .chars()
            .map(|c| c as i32)
            .filter(|c| *c > upper_end)
            .map(|c| (c, false))
            .collect::<HashMap<i32, bool>>();
        for c in word.chars() {
            let c = c as i32;
            if c <= upper_end && lowers.contains_key(&(c + diff)) {
                lowers.insert(c + diff, true);
            }
        }

        lowers.values().filter(|&v| *v).count() as i32
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
