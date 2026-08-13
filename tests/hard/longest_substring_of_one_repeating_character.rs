// 2213. Longest Substring of One Repeating Character
// https://leetcode.com/problems/longest-substring-of-one-repeating-character/

struct Solution;

impl Solution {
    pub fn longest_repeating(
        s: String,
        query_characters: String,
        query_indices: Vec<i32>,
    ) -> Vec<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::longest_substring_of_one_repeating_character::Solution;

    #[test]
    fn test_longest_repeating_1() {
        let s = "babacc".to_string();
        let query_characters = "bcb".to_string();
        let query_indices = [1, 3, 3].to_vec();
        let output = [3, 3, 4].to_vec();
        assert_eq!(
            output,
            Solution::longest_repeating(s, query_characters, query_indices)
        );
    }

    #[test]
    fn test_longest_repeating_2() {
        let s = "abyzz".to_string();
        let query_characters = "aa".to_string();
        let query_indices = [2, 1].to_vec();
        let output = [2, 3].to_vec();
        assert_eq!(
            output,
            Solution::longest_repeating(s, query_characters, query_indices)
        );
    }
}
