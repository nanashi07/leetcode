// 2452. Words Within Two Edits of Dictionary
// https://leetcode.com/problems/words-within-two-edits-of-dictionary/

struct Solution;

impl Solution {
    pub fn two_edit_words(queries: Vec<String>, dictionary: Vec<String>) -> Vec<String> {
        queries
            .into_iter()
            .filter(|q| {
                let qb = q.as_bytes();
                dictionary.iter().any(|d| {
                    qb.iter()
                        .zip(d.as_bytes())
                        .filter(|(a, b)| a != b)
                        .count()
                        <= 2
                })
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::words_within_two_edits_of_dictionary::Solution;
    use crate::shared::vec2d::to_string_vec;

    #[test]
    fn test_two_edit_words_1() {
        let queries = to_string_vec(["word", "note", "ants", "wood"]);
        let dictionary = to_string_vec(["wood", "joke", "moat"]);
        let output = to_string_vec(["word", "note", "wood"]);
        assert_eq!(output, Solution::two_edit_words(queries, dictionary));
    }

    #[test]
    fn test_two_edit_words_2() {
        let queries = to_string_vec(["yes"]);
        let dictionary = to_string_vec(["not"]);
        let output: Vec<String> = vec![];
        assert_eq!(output, Solution::two_edit_words(queries, dictionary));
    }
}
