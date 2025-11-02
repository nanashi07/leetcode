// 30. Substring with Concatenation of All Words
// https://leetcode.com/problems/substring-with-concatenation-of-all-words/
use std::collections::HashMap;
struct Solution;

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let len = s.len();
        let count = words.len();
        let size = words[0].len();
        let mut result = Vec::new();

        let map: HashMap<&str, i32> = words.iter().fold(HashMap::new(), |mut a, w| {
            let key = w.as_str();
            a.insert(key, a.get(key).unwrap_or(&0) + 1);
            a
        });

        for i in 0..(len - count * size + 1) {
            let str = &s[i..i + count * size];
            let mut seen: HashMap<&str, i32> = HashMap::new();
            let mut matched = 0;

            for wc in 0..count {
                let key = &str[wc * size..wc * size + size];
                if map.contains_key(key) {
                    seen.insert(key, seen.get(key).unwrap_or(&0) + 1);
                    if seen.get(key).unwrap_or(&0) > map.get(key).unwrap_or(&0) {
                        break;
                    }
                    matched = matched + 1;
                } else {
                    break;
                }
            }
            if matched == count {
                result.push(i as i32);
            }
        }

        result
    }

    // https://leetcode.com/problems/substring-with-concatenation-of-all-words/discuss/13658/Easy-Two-Map-Solution-(C%2B%2BJava)
    pub fn _find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let len = words[0].len();
        let num = words.len();
        let n = s.len();
        let counts: HashMap<&str, i32> = words.iter().fold(HashMap::new(), |mut a, w| {
            let key = w.as_str();
            a.insert(key, a.get(key).unwrap_or(&0) + 1);
            a
        });
        let mut indexes = Vec::new();

        for i in 0..(n - num * len + 1) {
            let mut seen: HashMap<&str, i32> = HashMap::new();
            let mut j = 0;
            while j < num {
                let word = &s[i + j * len..i + (j + 1) * len];
                if counts.contains_key(&word) {
                    let value = seen.get(word).unwrap_or(&0) + 1;
                    seen.insert(word, value);
                    if value > *counts.get(word).unwrap_or(&0) {
                        break;
                    }
                } else {
                    break;
                }
                j = j + 1;
            }
            if j == num {
                indexes.push(i as i32);
            }
        }

        indexes
    }

    pub fn _find_substring2(s: String, words: Vec<String>) -> Vec<i32> {
        let step = words[0].len();

        let mut result: Vec<i32> = Vec::new();

        for i in 0..words.len() {
            let mut words = words.clone();
            let w = words.remove(i);

            for pos in 0..s.len() {
                if pos + step <= s.len() && w[..] == s[pos..pos + step] {
                    // match next one
                    let matched = Solution::_match_next(pos + step, step, &s[pos + step..], &words);
                    // result matched, add to result
                    if matched && !result.contains(&(pos as i32)) {
                        result.push(pos as i32);
                    }
                }
            }
        }

        result
    }

    fn _match_next(p: usize, step: usize, s: &str, words: &Vec<String>) -> bool {
        if words.is_empty() {
            // matched
            return true;
        }

        for i in 0..words.len() {
            let mut words = words.clone();
            let w = words.remove(i);

            if s.len() >= step && w[..] == s[0..step] {
                // next one
                let matched = Solution::_match_next(p + step, step, &s[step..], &words);
                if matched {
                    return true;
                }
            }
        }

        false
    }
}

#[test]
fn test_find_substring() {
    let s = "barfoothefoobarman";
    let words = vec!["foo", "bar"]
        .into_iter()
        .map(|s| s.to_string())
        .collect();
    let mut result = Solution::find_substring(s.to_owned(), words);
    result.sort();
    assert_eq!(vec![0, 9], result);

    let s = "wordgoodgoodgoodbestword";
    let words = vec!["word", "good", "best", "word"]
        .into_iter()
        .map(|s| s.to_string())
        .collect();
    let expected: Vec<i32> = Vec::new();
    let mut result = Solution::find_substring(s.to_owned(), words);
    result.sort();
    assert_eq!(expected, result);

    let s = "barfoofoobarthefoobarman";
    let words = vec!["bar", "foo", "the"]
        .into_iter()
        .map(|s| s.to_string())
        .collect();
    let expected: Vec<i32> = vec![6, 9, 12];
    let mut result = Solution::find_substring(s.to_owned(), words);
    result.sort();
    assert_eq!(expected, result);

    let s = "wordgoodgoodgoodbestword";
    let words = vec!["word", "good", "best", "good"]
        .into_iter()
        .map(|s| s.to_string())
        .collect();
    let expected: Vec<i32> = vec![8];
    let mut result = Solution::find_substring(s.to_owned(), words);
    result.sort();
    assert_eq!(expected, result);

    let s = "lingmindraboofooowingdingbarrwingmonkeypoundcake";
    let words = vec!["fooo", "barr", "wing", "ding", "wing"]
        .into_iter()
        .map(|s| s.to_string())
        .collect();
    let expected: Vec<i32> = vec![13];
    let mut result = Solution::find_substring(s.to_owned(), words);
    result.sort();
    assert_eq!(expected, result);
}
