// 2785. Sort Vowels in a String
// https://leetcode.com/problems/sort-vowels-in-a-string/

struct Solution;

impl Solution {
    pub fn sort_vowels(s: String) -> String {
        let vowels = "aeiouAEIOU";

        // Extract all vowels from the string
        let mut extracted_vowels: Vec<char> = s.chars().filter(|&c| vowels.contains(c)).collect();

        // Sort the vowels
        extracted_vowels.sort();

        // Replace vowels in original string with sorted vowels
        let mut vowel_index = 0;
        let result: String = s
            .chars()
            .map(|c| {
                if vowels.contains(c) {
                    let vowel = extracted_vowels[vowel_index];
                    vowel_index += 1;
                    vowel
                } else {
                    c
                }
            })
            .collect();

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::sort_vowels_in_a_string::Solution;

    #[test]
    fn test_sort_vowels_1() {
        let s = "lEetcOde".to_owned();
        assert_eq!("lEOtcede".to_owned(), Solution::sort_vowels(s));
    }

    #[test]
    fn test_sort_vowels_2() {
        let s = "lYmpH".to_owned();
        assert_eq!("lYmpH".to_owned(), Solution::sort_vowels(s));
    }
}
