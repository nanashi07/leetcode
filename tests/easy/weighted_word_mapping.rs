// 3838. Weighted Word Mapping
// https://leetcode.com/problems/weighted-word-mapping/

struct Solution;

impl Solution {
    pub fn map_word_weights(words: Vec<String>, weights: Vec<i32>) -> String {
        words
            .iter()
            .map(|s| {
                char::from_u32(
                    'a' as u32 + 26
                        - 1
                        - (s.chars()
                            .map(|c| weights[c as usize - 'a' as usize])
                            .sum::<i32>() as u32
                            % 26),
                )
                .unwrap()
            })
            .collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::weighted_word_mapping::Solution;
    use crate::shared::vec2d::to_string_vec;

    #[test]
    fn test_map_word_weights_1() {
        let words = to_string_vec(["abcd", "def", "xyz"]);
        let weights = [
            5, 3, 12, 14, 1, 2, 3, 2, 10, 6, 6, 9, 7, 8, 7, 10, 8, 9, 6, 9, 9, 8, 3, 7, 7, 2,
        ]
        .to_vec();
        assert_eq!(
            "rij".to_string(),
            Solution::map_word_weights(words, weights)
        );
    }

    #[test]
    fn test_map_word_weights_2() {
        let words = to_string_vec(["a", "b", "c"]);
        let weights = [
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        ]
        .to_vec();
        assert_eq!(
            "yyy".to_string(),
            Solution::map_word_weights(words, weights)
        );
    }

    #[test]
    fn test_map_word_weights_3() {
        let words = to_string_vec(["abcd"]);
        let weights = [
            7, 5, 3, 4, 3, 5, 4, 9, 4, 2, 2, 7, 10, 2, 5, 10, 6, 1, 2, 2, 4, 1, 3, 4, 4, 5,
        ]
        .to_vec();
        assert_eq!("g".to_string(), Solution::map_word_weights(words, weights));
    }
}
