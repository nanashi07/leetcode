// 3014. Minimum Number of Pushes to Type Word I
// https://leetcode.com/problems/minimum-number-of-pushes-to-type-word-i/

struct Solution;

impl Solution {
    pub fn minimum_pushes(word: String) -> i32 {
        (0..word.len() as i32).map(|i| i / 8 + 1).sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::minimum_number_of_pushes_to_type_word_i::Solution;

    #[test]
    fn test_minimum_pushes_1() {
        let word = "abcde".to_string();
        assert_eq!(5, Solution::minimum_pushes(word));
    }

    #[test]
    fn test_minimum_pushes_2() {
        let word = "xycdefghij".to_string();
        assert_eq!(12, Solution::minimum_pushes(word));
    }
}
