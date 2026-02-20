// 761. Special Binary String
// https://leetcode.com/problems/special-binary-string/

struct Solution;

impl Solution {
    pub fn make_largest_special(s: String) -> String {
        if s.len() <= 2 {
            return s;
        }

        let mut count = 0;
        let mut i = 0;
        let mut subs: Vec<String> = Vec::new();
        let chars: Vec<char> = s.chars().collect();

        for j in 0..chars.len() {
            if chars[j] == '1' {
                count += 1;
            } else {
                count -= 1;
            }

            // When count == 0, we found a complete special substring
            if count == 0 {
                // Recursively process the inner part (exclude the outer '1' and '0')
                let inner = chars[i + 1..j].iter().collect::<String>();
                let processed = Self::make_largest_special(inner);
                // Wrap it back with '1' at start and '0' at end
                subs.push(format!("1{}0", processed));
                i = j + 1;
            }
        }

        // Sort in descending order to get lexicographically largest
        subs.sort_by(|a, b| b.cmp(a));
        subs.join("")
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::special_binary_string::Solution;

    #[test]
    fn test_make_largest_special_1() {
        let s = "11011000".to_string();
        assert_eq!("11100100".to_string(), Solution::make_largest_special(s));
    }

    #[test]
    fn test_make_largest_special_2() {
        let s = "10".to_string();
        assert_eq!("10".to_string(), Solution::make_largest_special(s));
    }
}
