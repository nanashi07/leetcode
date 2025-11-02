// 14. Longest Common Prefix
// https://leetcode.com/problems/longest-common-prefix/
struct Solution;
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut r = String::new();

        let mut i = 0;
        loop {
            let mut next_char: Option<char> = None;
            let mut count = 0;
            for s in strs.iter().filter(|s| s.len() > i) {
                if let Some(string_char) = s.chars().nth(i) {
                    if let Some(c) = next_char {
                        if c != string_char {
                            break;
                        }
                    } else {
                        next_char = Some(string_char);
                    }
                } else {
                    break;
                }
                count = count + 1;
            }

            if let Some(c) = next_char {
                if count == strs.len() {
                    r.push_str(&c.to_string())
                } else {
                    break;
                }
            } else {
                break;
            }

            i = i + 1;
        }

        r
    }
}

#[test]
fn test_longest_common_prefix() {
    let strs: Vec<String> = ["flower", "flow", "flight"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let result = Solution::longest_common_prefix(strs.clone());
    assert_eq!("fl".to_owned(), result, "strs = {:?}", &strs);

    let strs: Vec<String> = ["dog", "racecar", "car"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let result = Solution::longest_common_prefix(strs.clone());
    assert_eq!("".to_owned(), result, "strs = {:?}", &strs);
}
