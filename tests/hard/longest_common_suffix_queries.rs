// 3093. Longest Common Suffix Queries
// https://leetcode.com/problems/longest-common-suffix-queries/

struct Solution;

impl Solution {
    pub fn string_indices(words_container: Vec<String>, words_query: Vec<String>) -> Vec<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::longest_common_suffix_queries::Solution;
    use crate::shared::vec2d::to_string_vec;

    #[test]
    fn test_string_indices_1() {
        let words_container = to_string_vec(["abcd", "bcd", "xbcd"]);
        let words_query = to_string_vec(["cd", "bcd", "xyz"]);
        assert_eq!(
            [1, 1, 1].to_vec(),
            Solution::string_indices(words_container, words_query)
        );
    }

    #[test]
    fn test_string_indices_2() {
        let words_container = to_string_vec(["abcdefgh", "poiuygh", "ghghgh"]);
        let words_query = to_string_vec(["gh", "acbfgh", "acbfegh"]);
        assert_eq!(
            [2, 0, 2].to_vec(),
            Solution::string_indices(words_container, words_query)
        );
    }
}
