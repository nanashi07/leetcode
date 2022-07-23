// # 28. Implement strStr()
// https://leetcode.com/problems/implement-strstr/
struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let hl = haystack.len();
        let nl = needle.len();
        if nl <= hl {
            for i in 0..=hl - nl {
                if i + nl <= hl && &haystack[i..i + nl] == &needle[..] {
                    return i as i32;
                }
            }
        }
        -1
    }
}

#[test]
fn test_str_str() {
    let haystack = "hello";
    let needle = "ll";
    let result = Solution::str_str(haystack.to_owned(), needle.to_owned());
    assert_eq!(2, result, "haystack: {}, needle: {}", haystack, needle);

    let haystack = "aaaaa";
    let needle = "bba";
    let result = Solution::str_str(haystack.to_owned(), needle.to_owned());
    assert_eq!(-1, result, "haystack: {}, needle: {}", haystack, needle);

    let haystack = "a";
    let needle = "a";
    let result = Solution::str_str(haystack.to_owned(), needle.to_owned());
    assert_eq!(0, result, "haystack: {}, needle: {}", haystack, needle);

    let haystack = "aaa";
    let needle = "aaaa";
    let result = Solution::str_str(haystack.to_owned(), needle.to_owned());
    assert_eq!(-1, result, "haystack: {}, needle: {}", haystack, needle);
}
