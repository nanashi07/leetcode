// 1957. Delete Characters to Make Fancy String
// https://leetcode.com/problems/delete-characters-to-make-fancy-string/

struct Solution;

impl Solution {
    pub fn make_fancy_string(s: String) -> String {
        println!("s: {}", &s);
        if s.len() < 3 {
            return s;
        }
        let mut r: Vec<char> = Vec::new();
        for c in s.chars() {
            if r.len() >= 2 && r[r.len() - 1] == c && r[r.len() - 2] == c {
                continue;
            }
            r.push(c);
        }
        r.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::delete_characters_to_make_fancy_string::Solution;

    #[test]
    fn test_make_fancy_string_1() {
        let s = "leeetcode".to_owned();
        assert_eq!("leetcode".to_owned(), Solution::make_fancy_string(s));
    }

    #[test]
    fn test_make_fancy_string_2() {
        let s = "aaabaaaa".to_owned();
        assert_eq!("aabaa".to_owned(), Solution::make_fancy_string(s));
    }

    #[test]
    fn test_make_fancy_string_3() {
        let s = "aab".to_owned();
        assert_eq!("aab".to_owned(), Solution::make_fancy_string(s));
    }
}
