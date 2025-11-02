// 3. Longest Substring Without Repeating Characters
// https://leetcode.com/problems/longest-substring-without-repeating-characters/
use std::{cmp::max, collections::HashSet};

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut m = 0;
        for (i1, c1) in s.chars().enumerate() {
            let mut set: HashSet<char> = HashSet::new();
            set.insert(c1);

            for (_, c2) in s.char_indices().skip(i1 + 1) {
                if set.contains(&c2) {
                    break;
                } else {
                    set.insert(c2);
                }
            }

            m = max(m, set.len());
        }

        m as i32
    }
}

#[test]
fn test_length_of_longest_substring() {
    let input = "abcabcbb";
    let result = Solution::length_of_longest_substring(input.to_owned());
    assert_eq!(3, result, "input: {:?}", input);

    let input = "bbbbb";
    let result = Solution::length_of_longest_substring(input.to_owned());
    assert_eq!(1, result, "input: {:?}", input);

    let input = "pwwkew";
    let result = Solution::length_of_longest_substring(input.to_owned());
    assert_eq!(3, result, "input: {:?}", input);

    let input = " ";
    let result = Solution::length_of_longest_substring(input.to_owned());
    assert_eq!(1, result, "input: {:?}", input);

    let input = "abcabcbb";
    let result = Solution::length_of_longest_substring(input.to_owned());
    assert_eq!(3, result, "input: {:?}", input);

    let input = "au";
    let result = Solution::length_of_longest_substring(input.to_owned());
    assert_eq!(2, result, "input: {:?}", input);
}
