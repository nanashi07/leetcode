// 2273. Find Resultant Array After Removing Anagrams
// https://leetcode.com/problems/find-resultant-array-after-removing-anagrams/

struct Solution;

impl Solution {
    pub fn remove_anagrams(words: Vec<String>) -> Vec<String> {
        let mut result = Vec::new();
        let mut prev_sorted = String::new();

        for word in words {
            let mut chars: Vec<char> = word.chars().collect();
            chars.sort_unstable();
            let sorted: String = chars.into_iter().collect();

            if sorted != prev_sorted {
                result.push(word);
                prev_sorted = sorted;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::find_resultant_array_after_removing_anagrams::Solution;
    use crate::shared::vec2d::to_string_vec;

    #[test]
    fn test_remove_anagrams_1() {
        let words = to_string_vec(["abba", "baba", "bbaa", "cd", "cd"]);
        let output = to_string_vec(["abba", "cd"]);
        assert_eq!(output, Solution::remove_anagrams(words));
    }

    #[test]
    fn test_remove_anagrams_2() {
        let words = to_string_vec(["a", "b", "c", "d", "e"]);
        let output = to_string_vec(["a", "b", "c", "d", "e"]);
        assert_eq!(output, Solution::remove_anagrams(words));
    }
}
